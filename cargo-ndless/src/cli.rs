use clap::arg_enum;
use std::ffi::{OsStr, OsString};

use std::path::PathBuf;

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
		const AUTO: &[&str] = &["--color", "auto"];
		const ALWAYS: &[&str] = &["--color", "always"];
		const NEVER: &[&str] = &["--color", "never"];
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
	/// Compile the current package
	#[structopt(name = "build")]
	Build(Build),
	/// Compile the current package and send it to Firebird Emu
	#[structopt(name = "run")]
	Run(Run),
}

#[derive(Debug, StructOpt)]
pub struct Build {
	/// Path to Cargo.toml
	#[structopt(name = "PATH", long = "manifest-path", parse(from_os_str))]
	pub manifest_path: Option<PathBuf>,
	/// Coloring: auto, always, never
	#[structopt(long = "color", default_value = "auto")]
	pub color: Color,
	/// A target.json to compile for other than the built-in ndless toolchain
	#[structopt(long = "target", parse(from_os_str))]
	pub target: Option<OsString>,
	#[structopt(raw(raw = "true"), parse(from_os_str))]
	pub additional: Vec<OsString>,
}

#[derive(Debug, StructOpt)]
pub struct Run {
	/// Directory to send the tns files to firebird
	#[structopt(short, long, parse(from_os_str), default_value = "/ndless")]
	pub dest_dir: PathBuf,
	/// Port to connect to firebird
	#[structopt(short, long, default_value = "3334")]
	pub port: u16,
	#[structopt(flatten)]
	pub build_settings: Build,
}
