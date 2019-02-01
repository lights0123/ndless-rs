#![no_std]
#![feature(alloc)]
#![feature(maybe_uninit)]
extern crate alloc;

pub use bindings::*;

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::out::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	($($arg:tt)*) => ($crate::out::println(format!($($arg)*)));
}

pub mod prelude {
	pub use alloc::format;
	pub use alloc::string::{String, ToString};
	pub use alloc::vec;
	pub use alloc::vec::Vec;

}

mod bindings;

/// This macro takes a string and returns a CString
#[macro_export]
macro_rules! cstr {
	($str:expr) => { cstr_core::CString::new($str).unwrap() };
}
