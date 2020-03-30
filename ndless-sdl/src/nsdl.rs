//! # Module for managing nSDL fonts
//! Example:
//! ```
//! let font = Font::new(FontOptions::Thin, 255, 255, 255);
//! screen.draw_str(&font, "message", 0, 0);
//! ```
use ndless::prelude::*;

use crate::video::ll::SDL_Surface;

pub mod ll {
	use crate::video;

	#[repr(C)]
	pub struct nSDL_Font {
		pub chars: [*mut video::ll::SDL_Surface; 256usize],
		pub char_width: [cty::uint8_t; 256usize],
		pub hspacing: cty::c_int,
		pub vspacing: cty::c_int,
		pub monospaced: bool,
	}

	extern "C" {
		pub fn nSDL_LoadFont(font_index: i32, r: u8, g: u8, b: u8) -> *mut nSDL_Font;
		pub fn nSDL_DrawString(
			surface: *mut video::ll::SDL_Surface,
			font: *const nSDL_Font,
			x: i32,
			y: i32,
			str: *const cty::c_char,
		);
		pub fn nSDL_FreeFont(font: *mut nSDL_Font);
		pub fn nSDL_GetStringWidth(font: *mut nSDL_Font, str: *const cty::c_char) -> i32;
		pub fn nSDL_GetStringHeight(font: *mut nSDL_Font, str: *const cty::c_char) -> i32;
		pub fn nSDL_EnableFontMonospaced(font: *mut nSDL_Font, monospaced: i32);
		pub fn nSDL_SetFontSpacing(font: *mut nSDL_Font, hspacing: i32, vspacing: i32);
	}
}

#[repr(i32)]
pub enum FontOptions {
	Thin = 0,
	Space,
	VGA,
	Fantasy,
	ThinType,
}

pub struct Font {
	pub font: *mut ll::nSDL_Font,
}

impl Font {
	pub fn new(font: FontOptions, r: u8, g: u8, b: u8) -> Self {
		let font = unsafe { ll::nSDL_LoadFont(font as i32, r, g, b) };
		if font.is_null() {
			panic!("Error loading font")
		}
		Self { font }
	}

	pub fn draw(&self, surface: *mut SDL_Surface, msg: impl Into<String>, x: i32, y: i32) {
		let msg = ndless::cstr!(msg.into());
		unsafe { ll::nSDL_DrawString(surface, self.font, x, y, msg.as_ptr()) }
	}

	pub fn get_width(&self, msg: impl Into<String>) -> i32 {
		let msg = ndless::cstr!(msg.into());
		unsafe { ll::nSDL_GetStringWidth(self.font, msg.as_ptr()) }
	}

	pub fn get_height(&self, msg: impl Into<String>) -> i32 {
		let msg = ndless::cstr!(msg.into());
		unsafe { ll::nSDL_GetStringHeight(self.font, msg.as_ptr()) }
	}

	pub fn monospaced(&self, monospaced: bool) {
		unsafe { ll::nSDL_EnableFontMonospaced(self.font, if monospaced { 1 } else { 0 }) }
	}

	pub fn spacing(&self, horizontal: i32, vertical: i32) {
		unsafe { ll::nSDL_SetFontSpacing(self.font, horizontal, vertical) }
	}
}

impl Drop for Font {
	fn drop(&mut self) {
		unsafe { ll::nSDL_FreeFont(self.font) }
	}
}
