use crate::prelude::*;

#[lang = "eh_personality"]
extern fn eh_personality() {}


#[cfg(not(feature = "disable-oom-handler"))]
#[alloc_error_handler]
fn on_oom(_layout: core::alloc::Layout) -> ! {
	unsafe { ndless_sys::abort(); }
}


#[cfg(not(feature = "disable-panic-handler"))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	let msg = match info.message() {
		Some(err) => format!("An error occured: {}", err),
		None => format!("An error occured!")
	};
	let location = match info.location() {
		Some(loc) => format!("In file {} at line {} column {}", loc.file(), loc.line(), loc.column()),
		None => format!(""),
	};
	crate::msg::msg("Error", format!("{}\n{}", msg, location));
	unsafe { ndless_sys::abort() }
}
