//! # Getting input from hardware
//! This contains functionality to get keys pressed, as well as touchpad information.

use crate::bindings::input::raw_keys::*;
use crate::prelude::*;
use crate::alloc::borrow::Borrow;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum Key {
	Key0,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	Up,
	UpRight,
	Right,
	RightDown,
	Down,
	DownLeft,
	Left,
	LeftUp,
	Click,
	Catalog,
	Comma,
	Ctrl,
	Del,
	Divide,
	Doc,
	EE,
	Enter,
	Equals,
	Esc,
	Exponent,
	Flag,
	/// To the left of the catalog key
	Template,
	/// Also known as "Home"
	On,
	LeftParenthesis,
	Menu,
	Minus,
	Multiply,
	/// To the right of the period, left of enter
	Negative,
	Period,
	/// Under the EE, above the comma, to the left of the H
	Pi,
	Plus,
	/// Above the flag, to the right of the G. It has a question mark, exclamation mark, and
	/// a rightwards arrow.
	QuestionExclamation,
	Return,
	RightParenthesis,
	/// The calculator icon, under escape. Shown as *pad* in firebird-emu.
	Scratchpad,
	Shift,
	Space,
	/// x²
	Squared,
	Tab,
	/// 10ˣ
	TenExp,
	Trig,
	Var,
	/// eˣ
	EExp,

	/// Not available on TI-Nspire CX
	Apostrophe,
	/// Not available on TI-Nspire CX
	Bar,
	/// Not available on TI-Nspire CX
	Colon,
	/// Not available on TI-Nspire CX
	Cos,
	/// Not available on TI-Nspire CX
	GreaterThan,
	/// Not available on TI-Nspire CX
	II,
	/// Not available on TI-Nspire CX
	LessThan,
	/// Not available on TI-Nspire CX
	Question,
	/// Not available on TI-Nspire CX
	Quote,
	/// Not available on TI-Nspire CX
	Sin,
	/// Not available on TI-Nspire CX
	Tan,
	/// Not available on TI-Nspire CX
	Theta,
}

impl Key {
	fn from_arrow(arrow: u8) -> Option<Self> {
		match u32::from(arrow) {
			ndless_sys::tpad_arrow_TPAD_ARROW_CLICK => Some(Key::Click),
			ndless_sys::tpad_arrow_TPAD_ARROW_UP => Some(Key::Up),
			ndless_sys::tpad_arrow_TPAD_ARROW_UPRIGHT => Some(Key::UpRight),
			ndless_sys::tpad_arrow_TPAD_ARROW_RIGHT => Some(Key::Right),
			ndless_sys::tpad_arrow_TPAD_ARROW_RIGHTDOWN => Some(Key::RightDown),
			ndless_sys::tpad_arrow_TPAD_ARROW_DOWN => Some(Key::Down),
			ndless_sys::tpad_arrow_TPAD_ARROW_DOWNLEFT => Some(Key::DownLeft),
			ndless_sys::tpad_arrow_TPAD_ARROW_LEFT => Some(Key::Left),
			ndless_sys::tpad_arrow_TPAD_ARROW_LEFTUP => Some(Key::LeftUp),
			_ => None,
		}
	}
}

mod raw_keys {
	#![allow(non_camel_case_types)]
	#![allow(non_upper_case_globals)]

	const fn key(row: i32, col: i32) -> ndless_sys::t_key {
		ndless_sys::t_key {
			row,
			col,
			tpad_row: row,
			tpad_col: col,
			tpad_arrow: 0,
		}
	}

	const fn key_t_pad(row: i32, col: i32, tpad_row: i32, tpad_col: i32) -> ndless_sys::t_key {
		ndless_sys::t_key {
			row,
			col,
			tpad_row,
			tpad_col,
			tpad_arrow: 0,
		}
	}

	const fn key_t_pad_arrow(
		row: i32,
		col: i32,
		tpad_arrow: ndless_sys::tpad_arrow,
	) -> ndless_sys::t_key {
		ndless_sys::t_key {
			row,
			col,
			tpad_row: row,
			tpad_col: col,
			tpad_arrow,
		}
	}

	pub const _KEY_DUMMY_ROW: i32 = 0x1C;
	pub const _KEY_DUMMY_COL: i32 = 0x400;
	pub const KEY_NSPIRE_RET: ndless_sys::t_key = key(0x10, 0x001);
	pub const KEY_NSPIRE_ENTER: ndless_sys::t_key = key(0x10, 0x002);
	pub const KEY_NSPIRE_SPACE: ndless_sys::t_key = key_t_pad(0x10, 0x004, 0x10, 0x10);
	pub const KEY_NSPIRE_NEGATIVE: ndless_sys::t_key = key(0x10, 0x008);
	pub const KEY_NSPIRE_Z: ndless_sys::t_key = key_t_pad(0x10, 0x010, 0x10, 0x20);
	pub const KEY_NSPIRE_PERIOD: ndless_sys::t_key = key_t_pad(0x10, 0x20, 0x1A, 0x010);
	pub const KEY_NSPIRE_Y: ndless_sys::t_key = key(0x10, 0x040);
	pub const KEY_NSPIRE_0: ndless_sys::t_key = key(0x10, 0x080);
	pub const KEY_NSPIRE_X: ndless_sys::t_key = key_t_pad(0x10, 0x100, 0x12, 0x001);
	pub const KEY_NSPIRE_THETA: ndless_sys::t_key =
		key_t_pad(0x10, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_COMMA: ndless_sys::t_key = key_t_pad(0x12, 0x001, 0x1E, 0x400);
	pub const KEY_NSPIRE_PLUS: ndless_sys::t_key = key_t_pad(0x12, 0x002, 0x1C, 0x004);
	pub const KEY_NSPIRE_W: ndless_sys::t_key = key_t_pad(0x12, 0x004, 0x12, 0x002);
	pub const KEY_NSPIRE_3: ndless_sys::t_key = key(0x12, 0x008);
	pub const KEY_NSPIRE_V: ndless_sys::t_key = key_t_pad(0x12, 0x010, 0x12, 0x004);
	pub const KEY_NSPIRE_2: ndless_sys::t_key = key_t_pad(0x12, 0x020, 0x1C, 0x010);
	pub const KEY_NSPIRE_U: ndless_sys::t_key = key_t_pad(0x12, 0x040, 0x12, 0x010);
	pub const KEY_NSPIRE_1: ndless_sys::t_key = key(0x12, 0x080);
	pub const KEY_NSPIRE_T: ndless_sys::t_key = key_t_pad(0x12, 0x100, 0x12, 0x020);
	pub const KEY_NSPIRE_eEXP: ndless_sys::t_key = key_t_pad(0x12, 0x200, 0x16, 0x200);
	pub const KEY_NSPIRE_PI: ndless_sys::t_key = key_t_pad(0x12, 0x400, 0x12, 0x100);
	pub const KEY_NSPIRE_QUES: ndless_sys::t_key =
		key_t_pad(0x14, 0x001, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_QUESEXCL: ndless_sys::t_key =
		key_t_pad(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x10, 0x100);
	pub const KEY_NSPIRE_MINUS: ndless_sys::t_key = key_t_pad(0x14, 0x002, 0x1A, 0x004);
	pub const KEY_NSPIRE_S: ndless_sys::t_key = key_t_pad(0x14, 0x004, 0x12, 0x040);
	pub const KEY_NSPIRE_6: ndless_sys::t_key = key(0x14, 0x008);
	pub const KEY_NSPIRE_R: ndless_sys::t_key = key_t_pad(0x14, 0x010, 0x14, 0x001);
	pub const KEY_NSPIRE_5: ndless_sys::t_key = key_t_pad(0x14, 0x020, 0x1A, 0x040);
	pub const KEY_NSPIRE_Q: ndless_sys::t_key = key_t_pad(0x14, 0x040, 0x14, 0x002);
	pub const KEY_NSPIRE_4: ndless_sys::t_key = key(0x14, 0x080);
	pub const KEY_NSPIRE_P: ndless_sys::t_key = key_t_pad(0x14, 0x100, 0x14, 0x004);
	pub const KEY_NSPIRE_TENX: ndless_sys::t_key = key_t_pad(0x14, 0x200, 0x12, 0x400);
	pub const KEY_NSPIRE_EE: ndless_sys::t_key = key_t_pad(0x14, 0x400, 0x14, 0x100);
	pub const KEY_NSPIRE_COLON: ndless_sys::t_key =
		key_t_pad(0x16, 0x001, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_MULTIPLY: ndless_sys::t_key = key_t_pad(0x16, 0x002, 0x18, 0x100);
	pub const KEY_NSPIRE_O: ndless_sys::t_key = key_t_pad(0x16, 0x004, 0x14, 0x010);
	pub const KEY_NSPIRE_9: ndless_sys::t_key = key(0x16, 0x008);
	pub const KEY_NSPIRE_N: ndless_sys::t_key = key_t_pad(0x16, 0x010, 0x14, 0x020);
	pub const KEY_NSPIRE_8: ndless_sys::t_key = key_t_pad(0x16, 0x020, 0x1C, 0x040);
	pub const KEY_NSPIRE_M: ndless_sys::t_key = key_t_pad(0x16, 0x040, 0x14, 0x040);
	pub const KEY_NSPIRE_7: ndless_sys::t_key = key(0x16, 0x080);
	pub const KEY_NSPIRE_L: ndless_sys::t_key = key_t_pad(0x16, 0x100, 0x16, 0x001);
	pub const KEY_NSPIRE_SQU: ndless_sys::t_key = key_t_pad(0x16, 0x200, 0x14, 0x200);
	pub const KEY_NSPIRE_II: ndless_sys::t_key =
		key_t_pad(0x16, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_QUOTE: ndless_sys::t_key =
		key_t_pad(0x18, 0x001, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_DIVIDE: ndless_sys::t_key = key_t_pad(0x18, 0x002, 0x16, 0x100);
	pub const KEY_NSPIRE_K: ndless_sys::t_key = key_t_pad(0x18, 0x004, 0x16, 0x002);
	pub const KEY_NSPIRE_TAN: ndless_sys::t_key = key(0x18, 0x008);
	pub const KEY_NSPIRE_J: ndless_sys::t_key = key_t_pad(0x18, 0x010, 0x16, 0x004);
	pub const KEY_NSPIRE_COS: ndless_sys::t_key =
		key_t_pad(0x18, 0x020, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_I: ndless_sys::t_key = key_t_pad(0x18, 0x040, 0x16, 0x010);
	pub const KEY_NSPIRE_SIN: ndless_sys::t_key =
		key_t_pad(0x18, 0x080, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_H: ndless_sys::t_key = key_t_pad(0x18, 0x100, 0x16, 0x020);
	pub const KEY_NSPIRE_EXP: ndless_sys::t_key = key_t_pad(0x18, 0x200, 0x18, 0x200);
	pub const KEY_NSPIRE_GTHAN: ndless_sys::t_key =
		key_t_pad(0x18, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_APOSTROPHE: ndless_sys::t_key = key(0x1A, 0x001);
	pub const KEY_NSPIRE_CAT: ndless_sys::t_key = key_t_pad(0x1A, 0x002, 0x1A, 0x080);
	pub const KEY_NSPIRE_FRAC: ndless_sys::t_key =
		key_t_pad(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x1A, 0x100);
	pub const KEY_NSPIRE_G: ndless_sys::t_key = key_t_pad(0x1A, 0x004, 0x16, 0x040);
	pub const KEY_NSPIRE_RP: ndless_sys::t_key = key_t_pad(0x1A, 0x008, 0x1A, 0x008);
	pub const KEY_NSPIRE_F: ndless_sys::t_key = key_t_pad(0x1A, 0x010, 0x18, 0x001);
	pub const KEY_NSPIRE_LP: ndless_sys::t_key = key_t_pad(0x1A, 0x020, 0x1A, 0x020);
	pub const KEY_NSPIRE_E: ndless_sys::t_key = key_t_pad(0x1A, 0x040, 0x18, 0x002);
	pub const KEY_NSPIRE_VAR: ndless_sys::t_key = key_t_pad(0x1A, 0x080, 0x1A, 0x002);
	pub const KEY_NSPIRE_D: ndless_sys::t_key = key_t_pad(0x1A, 0x100, 0x18, 0x004);
	pub const KEY_NSPIRE_DEL: ndless_sys::t_key = key_t_pad(0x1E, 0x100, 0x1A, 0x200);
	pub const KEY_NSPIRE_LTHAN: ndless_sys::t_key =
		key_t_pad(0x1A, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_FLAG: ndless_sys::t_key = key(0x1C, 0x001);
	pub const KEY_NSPIRE_CLICK: ndless_sys::t_key =
		key_t_pad_arrow(0x1C, 0x002, ndless_sys::tpad_arrow_TPAD_ARROW_CLICK);
	pub const KEY_NSPIRE_C: ndless_sys::t_key = key_t_pad(0x1C, 0x004, 0x18, 0x010);
	pub const KEY_NSPIRE_HOME: ndless_sys::t_key = key_t_pad(0x1C, 0x008, 0x10, 0x200);
	pub const KEY_NSPIRE_B: ndless_sys::t_key = key_t_pad(0x1C, 0x010, 0x18, 0x020);
	pub const KEY_NSPIRE_MENU: ndless_sys::t_key = key(0x1C, 0x020);
	pub const KEY_NSPIRE_A: ndless_sys::t_key = key_t_pad(0x1C, 0x040, 0x18, 0x040);
	pub const KEY_NSPIRE_ESC: ndless_sys::t_key = key(0x1C, 0x080);
	pub const KEY_NSPIRE_BAR: ndless_sys::t_key = key(0x1C, 0x100);
	pub const KEY_NSPIRE_TAB: ndless_sys::t_key = key(0x1C, 0x200);
	pub const KEY_NSPIRE_EQU: ndless_sys::t_key = key_t_pad(0x1E, 0x400, 0x18, 0x080);
	pub const KEY_NSPIRE_UP: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x001, ndless_sys::tpad_arrow_TPAD_ARROW_UP);
	pub const KEY_NSPIRE_UPRIGHT: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x002, ndless_sys::tpad_arrow_TPAD_ARROW_UPRIGHT);
	pub const KEY_NSPIRE_RIGHT: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x004, ndless_sys::tpad_arrow_TPAD_ARROW_RIGHT);
	pub const KEY_NSPIRE_RIGHTDOWN: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x008, ndless_sys::tpad_arrow_TPAD_ARROW_RIGHTDOWN);
	pub const KEY_NSPIRE_DOWN: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x010, ndless_sys::tpad_arrow_TPAD_ARROW_DOWN);
	pub const KEY_NSPIRE_DOWNLEFT: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x020, ndless_sys::tpad_arrow_TPAD_ARROW_DOWNLEFT);
	pub const KEY_NSPIRE_LEFT: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x040, ndless_sys::tpad_arrow_TPAD_ARROW_LEFT);
	pub const KEY_NSPIRE_LEFTUP: ndless_sys::t_key =
		key_t_pad_arrow(0x1E, 0x080, ndless_sys::tpad_arrow_TPAD_ARROW_LEFTUP);
	pub const KEY_NSPIRE_SHIFT: ndless_sys::t_key = key_t_pad(0x1A, 0x200, 0x1E, 0x100);
	pub const KEY_NSPIRE_CTRL: ndless_sys::t_key = key(0x1E, 0x200);
	pub const KEY_NSPIRE_DOC: ndless_sys::t_key =
		key_t_pad(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x1C, 0x008);
	pub const KEY_NSPIRE_TRIG: ndless_sys::t_key =
		key_t_pad(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x12, 0x200);
	pub const KEY_NSPIRE_SCRATCHPAD: ndless_sys::t_key =
		key_t_pad(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x1A, 0x400);
}

const KEY_MAPPING: &[(ndless_sys::t_key, Key)] = &[
	(KEY_NSPIRE_0, Key::Key0),
	(KEY_NSPIRE_1, Key::Key1),
	(KEY_NSPIRE_2, Key::Key2),
	(KEY_NSPIRE_3, Key::Key3),
	(KEY_NSPIRE_4, Key::Key4),
	(KEY_NSPIRE_5, Key::Key5),
	(KEY_NSPIRE_6, Key::Key6),
	(KEY_NSPIRE_7, Key::Key7),
	(KEY_NSPIRE_8, Key::Key8),
	(KEY_NSPIRE_9, Key::Key9),
	(KEY_NSPIRE_A, Key::A),
	(KEY_NSPIRE_APOSTROPHE, Key::Apostrophe),
	(KEY_NSPIRE_B, Key::B),
	(KEY_NSPIRE_BAR, Key::Bar),
	(KEY_NSPIRE_C, Key::C),
	(KEY_NSPIRE_CAT, Key::Catalog),
	(KEY_NSPIRE_CLICK, Key::Click),
	(KEY_NSPIRE_COLON, Key::Colon),
	(KEY_NSPIRE_COMMA, Key::Comma),
	(KEY_NSPIRE_COS, Key::Cos),
	(KEY_NSPIRE_CTRL, Key::Ctrl),
	(KEY_NSPIRE_D, Key::D),
	(KEY_NSPIRE_DEL, Key::Del),
	(KEY_NSPIRE_DIVIDE, Key::Divide),
	(KEY_NSPIRE_DOC, Key::Doc),
	(KEY_NSPIRE_DOWN, Key::Down),
	(KEY_NSPIRE_DOWNLEFT, Key::DownLeft),
	(KEY_NSPIRE_E, Key::E),
	(KEY_NSPIRE_EE, Key::EE),
	(KEY_NSPIRE_ENTER, Key::Enter),
	(KEY_NSPIRE_EQU, Key::Equals),
	(KEY_NSPIRE_ESC, Key::Esc),
	(KEY_NSPIRE_EXP, Key::Exponent),
	(KEY_NSPIRE_F, Key::F),
	(KEY_NSPIRE_FLAG, Key::Flag),
	(KEY_NSPIRE_FRAC, Key::Template),
	(KEY_NSPIRE_G, Key::G),
	(KEY_NSPIRE_GTHAN, Key::GreaterThan),
	(KEY_NSPIRE_H, Key::H),
	(KEY_NSPIRE_HOME, Key::On),
	(KEY_NSPIRE_I, Key::I),
	(KEY_NSPIRE_II, Key::II),
	(KEY_NSPIRE_J, Key::J),
	(KEY_NSPIRE_K, Key::K),
	(KEY_NSPIRE_L, Key::L),
	(KEY_NSPIRE_LEFT, Key::Left),
	(KEY_NSPIRE_LEFTUP, Key::LeftUp),
	(KEY_NSPIRE_LP, Key::LeftParenthesis),
	(KEY_NSPIRE_LTHAN, Key::LessThan),
	(KEY_NSPIRE_M, Key::M),
	(KEY_NSPIRE_MENU, Key::Menu),
	(KEY_NSPIRE_MINUS, Key::Minus),
	(KEY_NSPIRE_MULTIPLY, Key::Multiply),
	(KEY_NSPIRE_N, Key::N),
	(KEY_NSPIRE_NEGATIVE, Key::Negative),
	(KEY_NSPIRE_O, Key::O),
	(KEY_NSPIRE_P, Key::P),
	(KEY_NSPIRE_PERIOD, Key::Period),
	(KEY_NSPIRE_PI, Key::Pi),
	(KEY_NSPIRE_PLUS, Key::Plus),
	(KEY_NSPIRE_Q, Key::Q),
	(KEY_NSPIRE_QUES, Key::Question),
	(KEY_NSPIRE_QUESEXCL, Key::QuestionExclamation),
	(KEY_NSPIRE_QUOTE, Key::Quote),
	(KEY_NSPIRE_R, Key::R),
	(KEY_NSPIRE_RET, Key::Return),
	(KEY_NSPIRE_RIGHT, Key::Right),
	(KEY_NSPIRE_RIGHTDOWN, Key::RightDown),
	(KEY_NSPIRE_RP, Key::RightParenthesis),
	(KEY_NSPIRE_S, Key::S),
	(KEY_NSPIRE_SCRATCHPAD, Key::Scratchpad),
	(KEY_NSPIRE_SHIFT, Key::Shift),
	(KEY_NSPIRE_SIN, Key::Sin),
	(KEY_NSPIRE_SPACE, Key::Space),
	(KEY_NSPIRE_SQU, Key::Squared),
	(KEY_NSPIRE_T, Key::T),
	(KEY_NSPIRE_TAB, Key::Tab),
	(KEY_NSPIRE_TAN, Key::Tan),
	(KEY_NSPIRE_TENX, Key::TenExp),
	(KEY_NSPIRE_THETA, Key::Theta),
	(KEY_NSPIRE_TRIG, Key::Trig),
	(KEY_NSPIRE_U, Key::U),
	(KEY_NSPIRE_UP, Key::Up),
	(KEY_NSPIRE_UPRIGHT, Key::UpRight),
	(KEY_NSPIRE_V, Key::V),
	(KEY_NSPIRE_VAR, Key::Var),
	(KEY_NSPIRE_W, Key::W),
	(KEY_NSPIRE_X, Key::X),
	(KEY_NSPIRE_Y, Key::Y),
	(KEY_NSPIRE_Z, Key::Z),
	(KEY_NSPIRE_eEXP, Key::EExp),
];

pub fn iter_keys() -> impl Iterator<Item = Key> + 'static {
	KEY_MAPPING
		.iter()
		.filter(|(raw_key, _key)| unsafe { ndless_sys::isKeyPressed(raw_key) } == 1)
		.map(|(_, key)| *key)
}

pub fn get_keys() -> Vec<Key> {
	iter_keys().collect()
}

pub fn is_key_pressed(key: impl Borrow<Key>) -> bool {
	KEY_MAPPING
		.iter()
		.find(|(_, other)| other == key.borrow())
		.map_or(
			false,
			|(raw_key, _key)| unsafe { ndless_sys::isKeyPressed(raw_key) } == 1,
		)
}

pub fn any_key_pressed() -> bool {
	unsafe { ndless_sys::any_key_pressed() > 0 }
}

pub fn key_on_pressed() -> bool {
	unsafe { ndless_sys::on_key_pressed() > 0 }
}

pub fn wait_key_pressed() {
	unsafe { ndless_sys::wait_key_pressed() }
}

pub fn wait_no_key_pressed() {
	unsafe { ndless_sys::wait_no_key_pressed() }
}

pub mod touchpad {
	pub use ndless_sys::touchpad_info_t as touchpad_info;
	use ndless_sys::touchpad_report_t as touchpad_report;

	use super::Key;

	#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
	pub struct TouchpadReport {
		pub contact: bool,
		pub proximity: u8,
		pub x: u16,
		pub y: u16,
		pub x_vel: u8,
		pub y_vel: u8,
		pub pressed: bool,
		pub arrow: Option<Key>,
	}

	impl From<touchpad_report> for TouchpadReport {
		fn from(report: touchpad_report) -> Self {
			Self {
				contact: report.contact > 0, // A C bool is often represented as a char with a nonzero value
				proximity: report.proximity,
				x: report.x,
				y: report.y,
				x_vel: report.x_velocity,
				y_vel: report.y_velocity,
				pressed: report.contact > 0,
				arrow: Key::from_arrow(report.arrow),
			}
		}
	}

	pub fn touchpad_scan() -> Result<TouchpadReport, i32> {
		let mut report = touchpad_report {
			contact: 0,
			proximity: 0,
			x: 0,
			y: 0,
			x_velocity: 0,
			y_velocity: 0,
			dummy: 0,
			pressed: 0,
			arrow: 0,
		};
		let status = unsafe { ndless_sys::touchpad_scan(&mut report) };
		match status {
			0 => Ok(report.into()),
			x => Err(x),
		}
	}

	pub fn get_touchpad_info() -> Result<touchpad_info, ()> {
		match unsafe { ndless_sys::touchpad_getinfo().as_ref() } {
			Some(ret) => Ok(*ret),
			None => Err(()),
		}
	}
}
