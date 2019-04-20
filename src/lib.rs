#![no_std]
#![feature(alloc_prelude)]
#![feature(core_intrinsics)]
#![feature(non_exhaustive)]
#![feature(asm)]
pub extern crate alloc;

pub use ndless_static_vars::ARGUMENTS;

pub use bindings::*;

mod bindings;

pub mod ffi {
    pub use core::ffi::*;

    pub use cstr_core::*;
}

pub use cty;

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (
		match write!($crate::out::STDOut {}, $($arg)*) {
			_ => {}
		}
	)
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
	($($arg:tt)*) => (
		match writeln!($crate::out::STDOut {}, $($arg)*) {
			_ => {}
		}
	)
}

#[macro_export]
macro_rules! dbg {
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!(
                    "[{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    stringify!($val),
                    &tmp
                );
                tmp
            }
        }
    };
}

pub mod prelude {
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
        cstr_core::CString::new($str).unwrap()
    };
}
