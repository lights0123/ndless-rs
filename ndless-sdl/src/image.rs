use cty::c_int;
use ndless::prelude::*;

use crate::get_error;
use crate::video::ll::SDL_RWFromConstMem;
use crate::video::Surface;

pub mod ll {
	#![allow(non_camel_case_types)]

	use cty::{c_char, c_int, c_uint};

	use crate::video::ll::{SDL_RWops, SDL_Surface};

	pub type IMG_InitFlags = c_uint;

	pub const IMG_INIT_JPG: IMG_InitFlags = 1;
	pub const IMG_INIT_PNG: IMG_InitFlags = 2;
	pub const IMG_INIT_TIF: IMG_InitFlags = 4;
	pub const IMG_INIT_WEBP: IMG_InitFlags = 8;

	extern "C" {
		pub fn IMG_Init(flags: c_int) -> c_int;
		pub fn IMG_Quit();
		pub fn IMG_Load(file: *const c_char) -> *mut SDL_Surface;
		pub fn IMG_LoadGIF_RW(src: *mut SDL_RWops) -> *mut SDL_Surface;
		pub fn IMG_LoadLBM_RW(src: *mut SDL_RWops) -> *mut SDL_Surface;
		pub fn IMG_LoadPCX_RW(src: *mut SDL_RWops) -> *mut SDL_Surface;
		pub fn IMG_LoadPNM_RW(src: *mut SDL_RWops) -> *mut SDL_Surface;
		pub fn IMG_LoadTGA_RW(src: *mut SDL_RWops) -> *mut SDL_Surface;
		pub fn IMG_LoadXCF_RW(src: *mut SDL_RWops) -> *mut SDL_Surface;
	}
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum InitFlag {
	JPG = ll::IMG_INIT_JPG as isize,
	PNG = ll::IMG_INIT_PNG as isize,
	TIF = ll::IMG_INIT_TIF as isize,
}

pub fn init(flags: &[InitFlag]) -> Vec<InitFlag> {
	let bitflags = unsafe {
		ll::IMG_Init(
			flags
				.iter()
				.fold(0i32, |flags, &flag| flags | flag as c_int),
		)
	};

	let flags = [InitFlag::JPG, InitFlag::PNG, InitFlag::TIF];

	flags
		.iter()
		.filter_map(|&flag| {
			if bitflags & (flag as c_int) != 0 {
				Some(flag)
			} else {
				None
			}
		})
		.collect()
}

pub fn load_file(file: impl Into<String>) -> Result<Surface, String> {
	let cfile = ndless::cstr!(file.into());
	unsafe {
		let raw = ll::IMG_Load(cfile.as_ptr());

		if raw.is_null() {
			Err(get_error())
		} else {
			Ok(Surface { raw, owned: true })
		}
	}
}

macro_rules! load_typed {
	($name: ident, $function: ident) => {
		pub fn $name(buffer: &[u8]) -> Result<Surface, String> {
			unsafe {
				let mem =
					SDL_RWFromConstMem(buffer.as_ptr() as *const cty::c_void, buffer.len() as i32);
				if mem.is_null() {
					return Err(get_error());
				}

				let raw = ll::$function(mem);

				if raw.is_null() {
					Err(get_error())
				} else {
					Ok(Surface { raw, owned: true })
				}
			}
		}
	};
}

load_typed!(load_mem_gif, IMG_LoadGIF_RW);
load_typed!(load_mem_lbm, IMG_LoadLBM_RW);
load_typed!(load_mem_pcx, IMG_LoadPCX_RW);
// PNM disabled for now due to possible link-time errors
//load_typed!(load_mem_pnm, IMG_LoadPNM_RW);
load_typed!(load_mem_tga, IMG_LoadTGA_RW);
load_typed!(load_mem_xcf, IMG_LoadXCF_RW);

pub fn quit() {
	unsafe {
		ll::IMG_Quit();
	}
}
