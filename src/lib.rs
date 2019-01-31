#![no_std]
#![feature(alloc)]
extern crate alloc;

use cty::c_void;

pub use bindings::*;

mod bindings;

pub mod prelude {
	pub use alloc::format;
	pub use alloc::string::{String, ToString};
	pub use alloc::vec;
	pub use alloc::vec::Vec;
}


/// This macro takes a string and returns a CString
#[macro_export]
macro_rules! cstr {
    ($str:expr) => { cstr_core::CString::new($str).unwrap() };
}
