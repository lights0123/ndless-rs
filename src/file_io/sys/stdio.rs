use core::mem::ManuallyDrop;

use crate::libc;

use super::super::io;
use super::super::sys::fd::FileDesc;

pub struct Stdout(());

impl Stdout {
	pub fn new() -> Stdout {
		Stdout(())
	}
}

impl io::Write for Stdout {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		ManuallyDrop::new(FileDesc::new(libc::STDOUT_FILENO)).write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}
