#![no_std]
#![feature(alloc)]
#![feature(core_intrinsics)]
#![feature(maybe_uninit)]
#![feature(asm)]
pub extern crate alloc;

pub use alloc::{borrow, boxed, collections, fmt, rc, slice, str, string, sync, vec};
pub use core::{
	any, arch, ascii, cell, char, clone, cmp, convert, default, f32, f64, hash, hint, i128, i16,
	i32, i64, i8, isize, iter, marker, mem, num, ops, option, ptr, result, u128, u16, u32,
	u64, u8,
};

pub use ndless_static_vars::ARGUMENTS;

pub use bindings::*;

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
	($($arg:tt)*) => ($crate::out::print(format!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	($($arg:tt)*) => ($crate::out::println($crate::prelude::format!($($arg)*)));
}

#[macro_export]
macro_rules! dbg {
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    }
}

pub mod prelude {
	pub use alloc::format;
	pub use alloc::prelude::*;
	pub use alloc::vec;

	pub use ndless_macros::entry;

	pub use print;
	pub use println;
	pub use dbg;

	pub use crate::math::Float;
}

/// This macro takes a string and returns a CString
#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        cstr_core::CString::new($str).unwrap()
    };
}
