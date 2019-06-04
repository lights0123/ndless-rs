//! # Message boxes
//! This module contains functions to display message boxes to the user.

use core::mem::transmute;
use core::slice;

use cstr_core::CStr;
use ndless_sys::_show_msgbox;

use crate::cstr;
use crate::prelude::*;

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Button {
	One = 1,
	Two = 2,
	Three = 3,
}

/// Creates a dialog box with a single button, labeled "OK"
pub fn msg(title: &str, msg: &str) {
	let title = cstr!(title);
	let msg = cstr!(msg);
	unsafe {
		_show_msgbox(title.as_ptr(), msg.as_ptr(), 0);
	}
}

/// Creates a dialog box with two buttons
pub fn msg_2b(title: &str, msg: &str, btn1: &str, btn2: &str) -> Button {
	let title = cstr!(title);
	let msg = cstr!(msg);
	let btn1 = cstr!(btn1);
	let btn2 = cstr!(btn2);
	unsafe {
		transmute(_show_msgbox(
			title.as_ptr(),
			msg.as_ptr(),
			2,
			btn1.as_ptr(),
			btn2.as_ptr(),
		))
	}
}

/// Creates a dialog box with three buttons
pub fn msg_3b(title: &str, msg: &str, btn1: &str, btn2: &str, btn3: &str) -> Button {
	let title = cstr!(title);
	let msg = cstr!(msg);
	let btn1 = cstr!(btn1);
	let btn2 = cstr!(btn2);
	let btn3 = cstr!(btn3);
	unsafe {
		transmute(_show_msgbox(
			title.as_ptr(),
			msg.as_ptr(),
			3,
			btn1.as_ptr(),
			btn2.as_ptr(),
			btn3.as_ptr(),
		))
	}
}

/// Creates a dialog box with a numerical input
pub fn msg_numeric(title: &str, subtitle: &str, msg: &str, range: (i32, i32)) -> Option<i32> {
	let title = cstr!(title);
	let subtitle = cstr!(subtitle);
	let msg = cstr!(msg);
	let mut num = 0i32;
	match unsafe {
		ndless_sys::show_1numeric_input(
			title.as_ptr(),
			subtitle.as_ptr(),
			msg.as_ptr(),
			&mut num,
			range.0,
			range.1,
		)
	} {
		1 => Some(num),
		_ => None,
	}
}

/// Creates a dialog box with two numerical inputs
pub fn msg_2numeric(
	title: &str,
	subtitle: &str,
	msg1: &str,
	range1: (i32, i32),
	msg2: &str,
	range2: (i32, i32),
) -> Option<(i32, i32)> {
	let title = cstr!(title);
	let subtitle = cstr!(subtitle);
	let msg1 = cstr!(msg1);
	let msg2 = cstr!(msg2);
	let mut num1 = 0i32;
	let mut num2 = 0i32;
	match unsafe {
		ndless_sys::show_2numeric_input(
			title.as_ptr(),
			subtitle.as_ptr(),
			msg1.as_ptr(),
			&mut num1,
			range1.0,
			range1.1,
			msg2.as_ptr(),
			&mut num2,
			range2.0,
			range2.1,
		)
	} {
		1 => Some((num1, num2)),
		_ => None,
	}
}

/// Creates a dialog box with a text input
pub fn msg_input(title: &str, msg: &str, default: &str) -> Option<String> {
	let title = cstr!(title);
	let msg = cstr!(msg);
	let default = cstr!(default);
	let mut ptr: *mut cty::c_char = core::ptr::null_mut();
	let ret = match unsafe {
		ndless_sys::show_msg_user_input(title.as_ptr(), msg.as_ptr(), default.as_ptr(), &mut ptr)
	} {
		-1 => None,
		len => unsafe {
			Some(
				CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(
					ptr as *const u8,
					len as usize + 1,
				))
				.to_string_lossy()
				.into_owned(),
			)
		},
	};
	unsafe { ndless_sys::free(ptr as *mut cty::c_void) };
	ret
}
