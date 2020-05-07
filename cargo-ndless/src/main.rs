use std::ffi::{OsStr, OsString};
use std::fs::remove_file;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::{env, process};

use anyhow::{ensure, Context, Result};
use cargo_metadata::Message;
use log::{debug, error, warn};
use serde::Deserialize;
use structopt::StructOpt;

use self::cli::Opt;
use crate::firebird::send_file;

mod cli;
mod files;
mod firebird;
mod install;

#[derive(Clone, Debug, Default, Deserialize)]
struct ZehnOptions {
	#[serde(default)]
	compress: bool,
	#[serde(default)]
	flags: String,
	name: Option<String>,
	notice: Option<String>,
}

fn update_path() {
	use std::iter::once;
	if let Some(ndless_home) = env::var_os("NDLESS_HOME") {
		let ndless_home = PathBuf::from(ndless_home);
		if let Some(path) = env::var_os("PATH") {
			debug!("Updating path with NDLESS_HOME...");
			let paths = once(ndless_home.join("ndless-sdk/toolchain/install/bin"))
				.chain(once(ndless_home.join("ndless-sdk/bin")))
				.chain(env::split_paths(&path));
			match env::join_paths(paths) {
				Ok(new_path) => env::set_var("PATH", &new_path),
				Err(e) => warn!("failed to create new PATH variable: {}", e),
			}
		}
	}
}

fn cargo_cmd() -> Command {
	let cmd = env::var_os("CARGO").map_or_else(|| PathBuf::from("cargo"), PathBuf::from);
	Command::new(cmd)
}

fn clean(manifest: Option<&Path>) -> io::Result<ExitStatus> {
	let mut cmd = cargo_cmd();
	cmd.arg("clean");
	if let Some(manifest) = manifest {
		cmd.arg("--manifest-path");
		cmd.arg(manifest);
	}
	cmd.status()
}

fn build_cmd<S: AsRef<OsStr>>(
	manifest: Option<&Path>,
	target: &OsStr,
	additional_args: impl IntoIterator<Item = S>,
) -> io::Result<Child> {
	let mut cmd = cargo_cmd();
	cmd.arg("xbuild");
	if let Some(manifest) = manifest {
		cmd.arg("--manifest-path");
		cmd.arg(manifest);
	}
	cmd.arg("--message-format=json-render-diagnostics")
		.arg("--target")
		.arg(target)
		.args(additional_args)
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.spawn()
}

fn build(build_settings: cli::Build) -> Result<(bool, Vec<PathBuf>)> {
	install::rustup_component("rust-src")?;
	install::cargo_plugin("xbuild")?;
	let mut some_failure = false;
	let target = files::get_target(build_settings.target.clone().map(PathBuf::from))?;
	if target.0 {
		ensure!(
			clean(build_settings.manifest_path.as_ref().map(AsRef::as_ref))?.success(),
			"cargo clean failed"
		)
	}
	let metadata = {
		let mut cmd = cargo_metadata::MetadataCommand::new();
		if let Some(ref path) = build_settings.manifest_path {
			cmd.manifest_path(path);
		}
		cmd.no_deps().exec().unwrap()
	};
	let mut command = build_cmd(
		build_settings.manifest_path.as_ref().map(AsRef::as_ref),
		(target.1).as_os_str(),
		build_settings
			.additional
			.iter()
			.map(OsString::as_os_str)
			.chain(build_settings.color.iter()),
	)?;
	let binaries = cargo_metadata::parse_messages(command.stdout.take().unwrap())
		.filter_map(|message| match message {
			Ok(Message::CompilerArtifact(artifact)) => Some(artifact),
			Ok(_) => None,
			Err(e) => {
				error!("{:#}", e);
				None
			}
		})
		.map(|artifact| {
			let binary = match artifact.executable {
				Some(ref binary) => binary,
				_ => return Ok(None),
			};
			let package = &metadata[&artifact.package_id];
			let config: ZehnOptions = package
				.metadata
				.get("zehn")
				.and_then(|m| serde_json::from_value(m.clone()).ok())
				.unwrap_or_default();
			let mut genzehn = Command::new("genzehn");
			let target_folder = binary.parent().unwrap();
			let zehn_file = target_folder.join(format!("{}.zehn", &package.name));
			genzehn
				.arg("--input")
				.arg(binary)
				.arg("--output")
				.arg(&zehn_file)
				.arg("--version")
				.arg(package.version.major.to_string())
				.arg("--name")
				.arg(config.name.as_ref().unwrap_or(&package.name))
				.args(config.flags.split(' '));
			if !package.authors.is_empty() {
				genzehn.arg("--author").arg(package.authors.join(", "));
			}
			if config.compress {
				genzehn.arg("--compress");
			}
			if let Some(ref notice) = config.notice {
				genzehn.arg("--notice").arg(notice);
			}
			ensure!(
				genzehn.status().context("Failed to run genzehn")?.success(),
				"Failed to run genzehn"
			);

			let mut make_prg = Command::new("make-prg");
			let tns_file = target_folder.join(format!("{}.tns", &package.name));
			make_prg.arg(&zehn_file.as_os_str()).arg(&tns_file);
			let make_prg_status = make_prg.status();
			remove_file(&zehn_file).unwrap_or(());
			ensure!(
				make_prg_status.context("Failed to run make-prg")?.success(),
				"Failed to run make-prg"
			);
			Ok(Some(tns_file))
		})
		.filter_map(|res| res.transpose())
		.filter_map(|res| match res {
			Err(err) => {
				some_failure = true;
				error!("{:#}", err);
				None
			}
			Ok(p) => Some(p),
		})
		.collect();
	Ok((some_failure, binaries))
}

fn main() -> Result<()> {
	env_logger::from_env(env_logger::Env::default().default_filter_or("warn")).init();
	if inner_main()? {
		process::exit(1);
	}
	Ok(())
}

fn inner_main() -> Result<bool> {
	let mut args = env::args_os().collect::<Vec<_>>();
	if let Some(true) = args.get(1).map(|arg| arg == "ndless") {
		args.remove(1);
	}
	let opt: Opt = Opt::from_iter(args.iter());
	update_path();
	match opt.cmd {
		cli::Command::Build(build_settings) => Ok(build(build_settings)?.0),
		cli::Command::Run(cli::Run {
			port,
			dest_dir,
			build_settings,
		}) => {
			let (mut some_failure, binaries) = build(build_settings)?;
			binaries
				.iter()
				.map(|binary| {
					send_file(port, &dest_dir, &binary).with_context(|| {
						binary.file_name().map_or_else(
							|| "Failed to send file".to_string(),
							|file| {
								format!(
									"Failed to send file {}",
									// Because apparently you can't .display() a &OsStr
									AsRef::<Path>::as_ref(file).display()
								)
							},
						)
					})
				})
				.filter_map(|res| res.err())
				.for_each(|err| {
					some_failure = true;
					error!("{:#}", err);
				});
			Ok(some_failure)
		}
	}
}
