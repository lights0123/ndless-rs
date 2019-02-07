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

pub fn is_classic() -> bool {
	unsafe { ndless_sys::hwtype() < 1 }
}

pub fn is_cm() -> bool {
	hw_subtype() == 1
}

pub fn has_colors() -> bool {
	!is_classic()
}

pub fn has_touchpad() -> bool {
	unsafe { ndless_sys::_is_touchpad() > 0 }
}

pub mod screen {
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
