//! Implementation of `std::os` functionality for unix systems

#![allow(unused_imports)] // lots of cfg code here

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;
use core::iter;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use core::slice;
use core::str;

use embedded_ffi::{CStr, CString, OsStr, OsString};

use libc::{c_char, c_int, c_void};

use crate::error::Error as StdError;
use crate::file_io::io;
use crate::file_io::memchr;
use crate::file_io::os::unix::prelude::*;
use crate::file_io::sys::cvt;
use crate::file_io::sys::fd;
use crate::libc;
use crate::path::{self, PathBuf};

const TMPBUF_SZ: usize = 128;

extern "C" {
	#[link_name = "__errno"]
	fn errno_location() -> *mut c_int;
}

/// Returns the platform-specific value of errno
pub fn errno() -> i32 {
	unsafe { (*errno_location()) as i32 }
}

pub fn set_errno(e: i32) {
	unsafe { *errno_location() = e as c_int }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(errno: i32) -> String {
	extern "C" {
		fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
	}

	let mut buf = [0 as c_char; TMPBUF_SZ];

	let p = buf.as_mut_ptr();
	unsafe {
		if strerror_r(errno as c_int, p, buf.len()) < 0 {
			panic!("strerror_r failure");
		}

		let p = p as *const _;
		str::from_utf8(CStr::from_ptr(p).to_bytes())
			.unwrap()
			.to_owned()
	}
}

pub fn getcwd() -> io::Result<PathBuf> {
	let mut buf = Vec::with_capacity(512);
	loop {
		unsafe {
			let ptr = buf.as_mut_ptr() as *mut libc::c_char;
			if !libc::getcwd(ptr, buf.capacity()).is_null() {
				let len = CStr::from_ptr(buf.as_ptr() as *const libc::c_char)
					.to_bytes()
					.len();
				buf.set_len(len);
				buf.shrink_to_fit();
				return Ok(PathBuf::from(OsString::from_vec(buf)));
			} else {
				let error = io::Error::last_os_error();
				if error.raw_os_error() != Some(libc::ERANGE) {
					return Err(error);
				}
			}

			// Trigger the internal buffer resizing logic of `Vec` by requiring
			// more space than the current capacity.
			let cap = buf.capacity();
			buf.set_len(cap);
			buf.reserve(1);
		}
	}
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
	let p: &OsStr = p.as_ref();
	let p = CString::new(p.as_bytes()).unwrap();
	unsafe {
		match libc::chdir(p.as_ptr()) == (0 as c_int) {
			true => Ok(()),
			false => Err(io::Error::last_os_error()),
		}
	}
}
