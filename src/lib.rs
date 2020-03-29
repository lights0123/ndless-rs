//! # ndless
//!
//! See [here] for examples. Additionally, don't forget to check out the [book].
//!
//! [here]: https://github.com/lights0123/example-nspire
//! [book]: https://lights0123.com/ndless-rust/index.html
#![no_std]
#![allow(clippy::tabs_in_doc_comments, clippy::needless_doctest_main)]
#![feature(alloc_prelude, allocator_api)]
#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(never_type)]
#![feature(maybe_uninit_ref)]
pub extern crate alloc;

pub use ndless_static_vars::ARGUMENTS;

pub use bindings::*;

mod bindings;
mod file_io;
mod libc;
pub use file_io::*;

pub mod ffi {
	pub use core::ffi::*;

	pub use embedded_ffi::*;
}

pub use cty;

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (
		match $crate::out::print_fmt(format_args!($($arg)*)) {
			_ => {}
		}
	)
}

#[macro_export]
macro_rules! println {
    () => ($crate::out::print("\n"));
	($($arg:tt)*) => (
		match $crate::out::print_fmt(format_args!("{}\n", format_args!($($arg)*))) {
			_ => {}
		}
	)
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

pub mod prelude {
	//! # Ndless prelude
	//! At the top of your code, add
	//! ```rust
	//! use ndless::prelude::*;
	//! ```
	//! to get commonly-used functions.
	pub use alloc::format;
	pub use alloc::prelude::v1::*;
	pub use alloc::vec;

	pub use ndless_macros::entry;

	pub use dbg;
	pub use print;
	pub use println;

	pub use crate::math::Float;
}

/// This macro takes a string and returns a CString
#[macro_export]
macro_rules! cstr {
	($str:expr) => {
		cstr_core::CString::new($str).expect("The passed string contains a null pointer")
	};
}
