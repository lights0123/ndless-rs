//! # Serial output
//! This module contains tools to output to the serial port. Also, take a look at
//! [io::stdout](crate::io::stdout).

use core::fmt::Arguments;

use crate::io;
use crate::io::Write;

pub fn print_fmt(fmt: Arguments) -> core::fmt::Result {
	io::stdout().write_fmt(fmt).map_err(|_| core::fmt::Error)
}
