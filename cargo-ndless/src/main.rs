use std::ffi::{OsStr, OsString};
use std::fs::remove_file;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};

use std::{env, process};

use anyhow::{ensure, Result};
use cargo_metadata::Message;

use serde::Deserialize;
use structopt::StructOpt;

use self::cli::Opt;

mod cli;
mod files;

#[derive(Clone, Debug, Default, Deserialize)]
struct ZehnOptions {
	#[serde(default)]
	compress: bool,
	#[serde(default)]
	flags: String,
	name: Option<String>,
	notice: Option<String>,
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

fn build_pkg<S: AsRef<OsStr>>(
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

fn main() -> Result<()> {
	if inner_main()? {
		process::exit(1);
	}
	Ok(())
}

fn inner_main() -> Result<bool> {
	let mut some_failure = false;
	let mut args = env::args_os().collect::<Vec<_>>();
	if let Some(true) = args.get(1).map(|arg| arg == "ndless") {
		args.remove(1);
	}
	let opt: Opt = Opt::from_iter(args.iter());
	match opt.cmd {
		cli::Command::Build(build_settings) => {
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
				cmd.exec().unwrap()
			};
			let mut command = build_pkg(
				build_settings.manifest_path.as_ref().map(AsRef::as_ref),
				(target.1).as_os_str(),
				build_settings
					.additional
					.iter()
					.map(OsString::as_os_str)
					.chain(build_settings.color.iter()),
			)?;
			for message in cargo_metadata::parse_messages(command.stdout.take().unwrap()) {
				if let Message::CompilerArtifact(artifact) = message.unwrap() {
					if let Some(ref binary) = artifact.executable {
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
						match genzehn.status() {
							Ok(status) => {
								if !status.success() {
									eprintln!("Failed to run genzehn");
									some_failure = true;
									continue;
								}
							}
							Err(e) => {
								eprintln!("Failed to run genzehn: {}", e);
								some_failure = true;
								continue;
							}
						}
						let mut make_prg = Command::new("make-prg");
						make_prg
							.arg(&zehn_file.as_os_str())
							.arg(target_folder.join(format!("{}.tns", &package.name)));
						let make_prg_status = make_prg.status();
						remove_file(&zehn_file).unwrap_or(());
						match make_prg_status {
							Ok(status) => {
								if !status.success() {
									eprintln!("Failed to run make-prg");
									some_failure = true;
									continue;
								}
							}
							Err(e) => {
								eprintln!("Failed to run make-prg: {}", e);
								some_failure = true;
								continue;
							}
						}
					}
				}
			}
		}
	}
	Ok(some_failure)
}
