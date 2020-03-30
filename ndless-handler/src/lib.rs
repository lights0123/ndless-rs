#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
extern crate alloc;

use alloc::format;
use alloc::string::ToString;

use crate::allocator::CAllocator;

mod allocator;

#[cfg(feature = "eh-personality")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(feature = "oom-handler")]
#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! {
	unsafe {
		ndless_sys::abort();
	}
}

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	{
		let msg = match info.message() {
			Some(err) => format!("An error occured: {}", err),
			None => "An error occured!".to_string(),
		};
		let location = match info.location() {
			Some(loc) => format!(
				"In file {} at line {} column {}",
				loc.file(),
				loc.line(),
				loc.column()
			),
			None => "".to_string(),
		};
		ndless::msg::msg("Error", &format!("{}\n{}", msg, location));
	}
	ndless::process::abort();
}

#[cfg(feature = "allocator")]
#[global_allocator]
static A: CAllocator = CAllocator;

#[cfg(feature = "ctype-ptr")]
#[no_mangle]
pub static __ctype_ptr__: [u8; 128 + 256] = [0; 128 + 256];
