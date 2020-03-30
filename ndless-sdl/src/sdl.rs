use core::str;
use cstr_core::{CStr, CString};
use ndless::prelude::*;

// Setup linking for all targets.
#[cfg(target_os = "macos")]
mod mac {
	#[cfg(mac_framework)]
	#[link(name = "SDL", kind = "framework")]
	extern "C" {}

	#[cfg(not(mac_framework))]
	#[link(name = "SDL")]
	extern "C" {}
}

#[cfg(not(target_os = "macos"))]
mod others {
	#[link(name = "SDL")]
	extern "C" {}
}

pub mod ll {
	#![allow(non_camel_case_types)]

	use cty::{c_char as c_schar, c_int, c_uint, uint32_t};
	pub type SDL_errorcode = c_uint;

	pub const SDL_ENOMEM: SDL_errorcode = 0;
	pub const SDL_EFREAD: SDL_errorcode = 1;
	pub const SDL_EFWRITE: SDL_errorcode = 2;
	pub const SDL_EFSEEK: SDL_errorcode = 3;
	pub const SDL_UNSUPPORTED: SDL_errorcode = 4;
	pub const SDL_LASTERROR: SDL_errorcode = 5;

	pub type SDL_InitFlag = uint32_t;

	pub const SDL_INIT_TIMER: SDL_InitFlag = 0x0000_0001;
	pub const SDL_INIT_AUDIO: SDL_InitFlag = 0x0000_0010;
	pub const SDL_INIT_VIDEO: SDL_InitFlag = 0x0000_0020;
	pub const SDL_INIT_CDROM: SDL_InitFlag = 0x0000_0100;
	pub const SDL_INIT_JOYSTICK: SDL_InitFlag = 0x0000_0200;
	pub const SDL_INIT_NOPARACHUTE: SDL_InitFlag = 0x0010_0000;
	pub const SDL_INIT_EVENTTHREAD: SDL_InitFlag = 0x0100_0000;
	pub const SDL_INIT_EVERYTHING: SDL_InitFlag = 0x0000_FFFF;

	extern "C" {
		pub fn SDL_ClearError();
		pub fn SDL_Error(code: SDL_errorcode);
		pub fn SDL_SetError(fmt: *const c_schar);
		pub fn SDL_GetError() -> *mut c_schar;
		pub fn SDL_Quit();
		pub fn SDL_QuitSubSystem(flags: SDL_InitFlag);
		pub fn SDL_Init(flags: uint32_t) -> c_int;
		pub fn SDL_InitSubSystem(flags: SDL_InitFlag) -> c_int;
		pub fn SDL_WasInit(flags: SDL_InitFlag) -> SDL_InitFlag;
		pub fn SDL_GetTicks() -> uint32_t;
	}
}

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct Rect {
	pub x: i16,
	pub y: i16,
	pub w: u16,
	pub h: u16,
}

#[allow(non_snake_case)]
pub fn Rect(x: i16, y: i16, w: u16, h: u16) -> Rect {
	Rect { x, y, w, h }
}

impl Rect {
	pub fn new(x: i16, y: i16, w: u16, h: u16) -> Rect {
		Rect { x, y, w, h }
	}
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum InitFlag {
	Timer = ll::SDL_INIT_TIMER as isize,
	Audio = ll::SDL_INIT_AUDIO as isize,
	Video = ll::SDL_INIT_VIDEO as isize,
	CDRom = ll::SDL_INIT_CDROM as isize,
	Joystick = ll::SDL_INIT_JOYSTICK as isize,
	NoParachute = ll::SDL_INIT_NOPARACHUTE as isize,
	EventThread = ll::SDL_INIT_EVENTTHREAD as isize,
	Everything = ll::SDL_INIT_EVERYTHING as isize,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Error {
	NoMem = ll::SDL_ENOMEM as isize,
	Read = ll::SDL_EFREAD as isize,
	Write = ll::SDL_EFWRITE as isize,
	Seek = ll::SDL_EFSEEK as isize,
	Unsupported = ll::SDL_UNSUPPORTED as isize,
}

pub fn init(flags: &[InitFlag]) -> bool {
	unsafe {
		ll::SDL_Init(
			flags
				.iter()
				.fold(0u32, |flags, &flag| flags | flag as ll::SDL_InitFlag),
		) == 0
	}
}

pub fn init_subsystem(flags: &[InitFlag]) -> bool {
	unsafe {
		ll::SDL_InitSubSystem(
			flags
				.iter()
				.fold(0u32, |flags, &flag| flags | flag as ll::SDL_InitFlag),
		) == 0
	}
}

pub fn quit_subsystem(flags: &[InitFlag]) {
	let flags = flags
		.iter()
		.fold(0u32, |flags, &flag| flags | flag as ll::SDL_InitFlag);

	unsafe {
		ll::SDL_QuitSubSystem(flags);
	}
}

pub fn quit() {
	unsafe {
		ll::SDL_Quit();
	}
}

pub fn was_inited(flags: &[InitFlag]) -> Vec<InitFlag> {
	let flags = flags
		.iter()
		.fold(0u32, |flags, &flag| flags | flag as ll::SDL_InitFlag);
	let bitflags = unsafe { ll::SDL_WasInit(flags) };

	let flags = [
		InitFlag::Timer,
		InitFlag::Audio,
		InitFlag::Video,
		InitFlag::CDRom,
		InitFlag::Joystick,
		InitFlag::NoParachute,
		InitFlag::EventThread,
		InitFlag::Everything,
	];

	flags
		.iter()
		.filter_map(|&flag| {
			if bitflags & (flag as ll::SDL_InitFlag) != 0 {
				Some(flag)
			} else {
				None
			}
		})
		.collect()
}

pub fn get_error() -> String {
	unsafe {
		let cstr = ll::SDL_GetError() as *const cty::c_char;
		let slice = CStr::from_ptr(cstr).to_bytes();

		str::from_utf8(slice).unwrap().to_string()
	}
}

pub fn set_error(err: &str) {
	unsafe {
		ll::SDL_SetError(CString::new(err.as_bytes()).unwrap().as_ptr());
	}
}

pub fn set_error_from_code(err: Error) {
	unsafe { ll::SDL_Error(err as ll::SDL_errorcode) }
}

pub fn clear_error() {
	unsafe {
		ll::SDL_ClearError();
	}
}

pub fn get_ticks() -> usize {
	unsafe { ll::SDL_GetTicks() as usize }
}
