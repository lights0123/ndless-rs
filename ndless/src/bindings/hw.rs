//! # Tools to interact with the hardware
//! This module contains functions to gather information about the calculator.

/// Returned by [`hw_type`]
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum Type {
	Nspire,
	NspireCX,
	Future(u32),
}

pub fn hw_type() -> Type {
	match unsafe { ndless_sys::hwtype() } {
		0 => Type::Nspire,
		1 => Type::NspireCX,
		future => Type::Future(future),
	}
}

pub fn hw_subtype() -> u32 {
	unsafe { ndless_sys::nl_hwsubtype() }
}

/// since Ndless v3.1. TRUE on classic TI-Nspire.
///
/// This is the preferred way to check CX/CM-specific features.
pub fn is_classic() -> bool {
	unsafe { ndless_sys::hwtype() < 1 }
}

/// since Ndless v3.1 r863. TRUE on TI-Nspire CM/CM-C.
pub fn is_cm() -> bool {
	hw_subtype() == 1
}

/// since Ndless v3.1. TRUE if the device has a screen in color.
pub fn has_colors() -> bool {
	!is_classic()
}

/// TRUE on a TI-Nspire Touchpad or on a TI-Nspire CX.
pub fn has_touchpad() -> bool {
	unsafe { ndless_sys::_is_touchpad() > 0 }
}

/// flush the data cache and invalidate the instruction and data caches of the processor. Should be
/// called before loading code dynamically, after a code patch or with self-modifying code.
pub fn clear_cache() {
	unsafe { ndless_sys::clear_cache() }
}

pub mod screen {
	/// Returned by [`lcd_type`]
	#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
	pub enum Screen {
		/// 4bit grayscale. Native on classic calcs.
		Screen320x240x4,
		/// 8bit paletted mode.
		Screen320x240x8,
		/// RGB444
		Screen320x240x16,
		/// RGB565. Native on CX before HW-W
		Screen320x240x565,
		/// RGB565. Native on CX HW-W
		Screen240x320x565,
		Screen320x240x555,
		Screen240x320x555,
		Unknown,
	}

	pub fn lcd_type() -> Screen {
		match unsafe { ndless_sys::lcd_type() } {
			ndless_sys::scr_type_t_SCR_320x240_4 => Screen::Screen320x240x4,
			ndless_sys::scr_type_t_SCR_320x240_8 => Screen::Screen320x240x8,
			ndless_sys::scr_type_t_SCR_320x240_16 => Screen::Screen320x240x16,
			ndless_sys::scr_type_t_SCR_320x240_565 => Screen::Screen320x240x565,
			ndless_sys::scr_type_t_SCR_240x320_565 => Screen::Screen240x320x565,
			ndless_sys::scr_type_t_SCR_320x240_555 => Screen::Screen320x240x555,
			ndless_sys::scr_type_t_SCR_240x320_555 => Screen::Screen240x320x555,
			_ => Screen::Unknown,
		}
	}
}

/// Go to sleep until an interrupt occurs
pub fn idle() {
	unsafe { ndless_sys::idle() }
}
