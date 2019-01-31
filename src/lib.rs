#![no_std]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

use cty::c_void;

pub use bindings::*;

mod handlers;
mod bindings;

pub mod prelude {
	pub use alloc::format;
	pub use alloc::string::{String, ToString};
	pub use alloc::vec;
	pub use alloc::vec::Vec;
}

/// This allows for dynamic allocation, which calls the C functions `calloc` and `free`.
struct CAllocator;

unsafe impl GlobalAlloc for CAllocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		ndless_sys::calloc(1, layout.size()) as *mut u8
	}
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
		ndless_sys::free(ptr as *mut c_void)
	}
}

#[cfg(not(feature = "disable-allocator"))]
#[global_allocator]
static A: CAllocator = CAllocator;

/// This macro takes a string and returns a CString
#[macro_export]
macro_rules! cstr {
    ($str:expr) => { cstr_core::CString::new($str).unwrap() };
}
