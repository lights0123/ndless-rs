use clap::arg_enum;
use std::ffi::{OsStr, OsString};
use std::iter;
use std::path::PathBuf;
use std::slice;
use structopt::clap::AppSettings;
use structopt::StructOpt;

arg_enum! {
	#[derive(Copy, Clone, Debug)]
	pub enum Color {
		Auto,
		Always,
		Never,
	}
}
impl Color {
	pub fn iter(&self) -> impl Iterator<Item = &OsStr> {
		const AUTO: &'static [&'static str] = &["--color", "auto"];
		const ALWAYS: &'static [&'static str] = &["--color", "always"];
		const NEVER: &'static [&'static str] = &["--color", "never"];
		match self {
			Color::Auto => AUTO,
			Color::Always => ALWAYS,
			Color::Never => NEVER,
		}
		.iter()
		.map(OsStr::new)
	}
}
#[derive(Debug, StructOpt)]
pub struct Opt {
	#[structopt(subcommand)]
	pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
	#[structopt(name = "build")]
	/// Compile the current package
	Build(Build),
}

#[derive(Debug, StructOpt)]
pub struct Build {
	#[structopt(name = "PATH", long = "manifest-path", parse(from_os_str))]
	pub manifest_path: Option<PathBuf>,
	/// Coloring: auto, always, never
	#[structopt(long = "color", default_value = "auto")]
	pub color: Color,
	#[structopt(long = "target", parse(from_os_str))]
	pub target: Option<OsString>,
	#[structopt(raw(raw = "true"), parse(from_os_str))]
	pub additional: Vec<OsString>,
}
