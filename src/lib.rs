#![no_std]
#![feature(alloc)]
#![feature(core_intrinsics)]
pub extern crate alloc;

pub use alloc::{borrow, boxed, collections, fmt, rc, slice, str, string, sync, vec};
pub use core::{
	any, arch, ascii, cell, char, clone, cmp, convert, default, f32, f64, hash, hint, i128, i16,
	i32, i64, i8, isize, iter, marker, mem, num, ops, option, ptr, result, u128, u16, u32,
	u64, u8,
};

pub use bindings::*;
pub use ndless_static_vars::ARGUMENTS;

mod bindings;

pub mod ffi {
	pub use core::ffi::*;

	pub use cstr_core::*;
}

pub mod cty {
	pub use cty::*;
}

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
	pub use alloc::prelude::*;
	pub use alloc::vec;

	pub use ndless_macros::entry;
	pub use print;
	pub use println;

	pub use crate::math::Float;
}

/// This macro takes a string and returns a CString
#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        cstr_core::CString::new($str).unwrap()
    };
}
