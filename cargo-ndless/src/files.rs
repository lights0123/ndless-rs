use std::env;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use log::debug;

type GenericPath = Box<dyn Deref<Target = Path>>;

struct TemporaryFile {
	path: PathBuf,
}

impl Deref for TemporaryFile {
	type Target = Path;

	fn deref(&self) -> &Self::Target {
		&self.path
	}
}

impl Drop for TemporaryFile {
	fn drop(&mut self) {
		if let Err(e) = remove_dir_all(&self.path) {
			if std::thread::panicking() {
				eprintln!("Could not remove path {:?}: {}", self.path, e);
			} else {
				panic!("Could not remove path {:?}: {}", self.path, e);
			}
		}
	}
}

pub fn get_file(filename: impl AsRef<Path>, wanted_contents: &[u8]) -> Result<(bool, GenericPath)> {
	let filename = filename.as_ref();
	if let Some(mut path) = dirs::home_dir() {
		path.push(".ndless");
		path.push(filename);
		if let Ok(mut file) = File::open(&path) {
			let mut contents = vec![];
			if file.read_to_end(&mut contents).is_ok() && contents == wanted_contents {
				return Ok((false, Box::new(path)));
			}
			debug!("Updating {}...", filename.display());
		}
		if create_dir_all(path.parent().unwrap()).is_ok() {
			if let Ok(mut file) = File::create(&path) {
				if file.write_all(wanted_contents).is_ok() {
					return Ok((true, Box::new(path)));
				}
			}
		}
		debug!(
			"Couldn't create {} in home directory. Read-only filesystem?",
			filename.display()
		);
	} else {
		debug!("Couldn't find home directory");
	}
	let mut path = env::temp_dir();
	path.push(filename);
	if let Ok(mut file) = File::open(&path) {
		let mut contents = vec![];
		if file.read_to_end(&mut contents).is_ok() && contents == wanted_contents {
			return Ok((false, Box::new(path)));
		}
		debug!("Updating {} in temp directory...", filename.display());
	}
	if let Ok(mut file) = File::create(&path) {
		if file.write_all(wanted_contents).is_ok() {
			return Ok((true, Box::new(path)));
		}
	}
	bail!(
		"Couldn't create {} in temp directory. Read-only filesystem?",
		filename.display()
	)
}

pub fn get_target(existing: Option<PathBuf>) -> Result<(bool, GenericPath)> {
	if let Some(existing) = existing {
		Ok((false, Box::new(existing)))
	} else {
		get_file(
			"armv5te-nspire-eabi.json",
			include_bytes!("armv5te-nspire-eabi.json"),
		)
	}
}
