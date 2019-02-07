pub fn abort() -> ! {
	unsafe { ndless_sys::abort() }
}

pub fn exit(code: i32) -> ! {
	unsafe { ndless_sys::exit(code) }
}
