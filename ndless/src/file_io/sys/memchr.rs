// Original implementation taken from rust-memchr.
// Copyright 2015 Andrew Gallant, bluss and Nicolas Koch

use crate::libc;
pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
	let p = unsafe {
		libc::memchr(
			haystack.as_ptr() as *const libc::c_void,
			i32::from(needle),
			haystack.len(),
		)
	};
	if p.is_null() {
		None
	} else {
		Some(p as usize - (haystack.as_ptr() as usize))
	}
}

/// Returns the last index matching the byte `x` in `text`.
pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
	if haystack.is_empty() {
		return None;
	}
	let p = unsafe {
		libc::memrchr(
			haystack.as_ptr() as *const libc::c_void,
			i32::from(needle),
			haystack.len(),
		)
	};
	if p.is_null() {
		None
	} else {
		Some(p as usize - (haystack.as_ptr() as usize))
	}
}
