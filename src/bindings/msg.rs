use ndless_sys::_show_msgbox;
use crate::prelude::*;
use crate::cstr;
use core::mem::transmute;
#[repr(u32)]
#[derive(Debug)]
pub enum Button {
	ONE = 1,
	TWO = 2,
	THREE = 3,
}

pub fn msg(title: impl Into<String>, msg: impl Into<String>) {
	let title = cstr!(title.into());
	let msg = cstr!(msg.into());
	unsafe {
		_show_msgbox(title.as_ptr(), msg.as_ptr(), 0);
	}
}

pub fn msg_2b(
	title: impl Into<String>,
	msg: impl Into<String>,
	btn1: impl Into<String>,
	btn2: impl Into<String>,
) -> Button {
	let title = cstr!(title.into());
	let msg = cstr!(msg.into());
	let btn1 = cstr!(btn1.into());
	let btn2 = cstr!(btn2.into());
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
pub fn msg_3b(
	title: impl Into<String>,
	msg: impl Into<String>,
	btn1: impl Into<String>,
	btn2: impl Into<String>,
	btn3: impl Into<String>,
) -> Button {
	let title = cstr!(title.into());
	let msg = cstr!(msg.into());
	let btn1 = cstr!(btn1.into());
	let btn2 = cstr!(btn2.into());
	let btn3 = cstr!(btn3.into());
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
