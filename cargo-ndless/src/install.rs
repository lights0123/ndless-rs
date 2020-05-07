use super::cargo_cmd;
use anyhow::{bail, Result};
use log::debug;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
pub fn rustup_component(name: impl AsRef<OsStr>) -> Result<()> {
	let mut cmd = Command::new("rustup");
	let name = name.as_ref();
	cmd.arg("component").arg("add").arg(name);
	cmd.status().map_err(Into::into).and_then(|status| {
		if status.success() {
			debug!(
				"Successfully installed {} via Rustup",
				AsRef::<Path>::as_ref(name).display()
			);
			Ok(())
		} else {
			match status.code() {
				Some(code) => bail!("Failed to run rustup: error code {}", code),
				None => bail!("Failed to run rustup"),
			}
		}
	})
}

fn is_cargo_command_installed(name: &str) -> Result<bool> {
	let output = cargo_cmd().arg("--list").output()?;
	if !output.status.success() {
		match output.status.code() {
			Some(code) => bail!("Failed to run rustup: error code {}", code),
			None => bail!("Failed to run rustup"),
		}
	}
	let stdout = String::from_utf8_lossy(&output.stdout);
	Ok(stdout.lines().map(str::trim).any(|line| line == name))
}

pub fn cargo_plugin(name: &str) -> Result<()> {
	if is_cargo_command_installed(name)? {
		return Ok(());
	}
	let status = cargo_cmd()
		.arg("install")
		.arg(format!("cargo-{}", name))
		.status()?;
	if !status.success() {
		match status.code() {
			Some(code) => bail!("Failed to run cargo install {}: error code {}", name, code),
			None => bail!("Failed to run cargo install {}", name),
		}
	}
	Ok(())
}
