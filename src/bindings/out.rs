use crate::prelude::*;
use crate::cstr;
pub fn print(msg: impl Into<String>) {
	let _msg = cstr!(msg.into());
	/*if unsafe { ndless_sys::fputs(msg.as_ptr(), stdout) } < 0 {
		panic!("Error writing to stdout");
	}*/
	unimplemented!()
}

pub fn println(msg: impl Into<String>) {
	let msg = cstr!(msg.into());
	if unsafe { ndless_sys::puts(msg.as_ptr()) } < 0 {
		panic!("Error writing to stdout");
	}
}
