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
