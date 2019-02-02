use crate::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Key {
	KEY_0,
	KEY_1,
	KEY_2,
	KEY_3,
	KEY_4,
	KEY_5,
	KEY_6,
	KEY_7,
	KEY_8,
	KEY_9,
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
	UP,
	UPRIGHT,
	RIGHT,
	RIGHTDOWN,
	DOWN,
	DOWNLEFT,
	LEFT,
	LEFTUP,
	CLICK,
	CATALOG,
	COMMA,
	CTRL,
	DEL,
	DIVIDE,
	DOC,
	EE,
	ENTER,
	EQUALS,
	ESC,
	EXPONENT,
	FLAG,
	/// To the left of the catalog key
	TEMPLATE,
	/// Also known as "Home"
	ON,
	LEFT_PARENTHESIS,
	MENU,
	MINUS,
	MULTIPLY,
	/// To the right of the period, left of enter
	NEGATIVE,
	PERIOD,
	/// Under the EE, above the comma, to the left of the H
	PI,
	PLUS,
	/// Above the flag, to the right of the G. It has a question mark, exclamation mark, and
	/// a rightwards arrow.
	QUESTION_EXCLAMATION,
	RETURN,
	RIGHT_PARENTHESIS,
	/// The calculator icon, under escape. Shown as *pad* in firebird-emu.
	SCRATCHPAD,
	SHIFT,
	SPACE,
	/// x²
	SQUARED,
	TAB,
	/// 10ˣ
	TEN_EXP,
	TRIG,
	VAR,
	/// eˣ
	E_EXP,

	/// Not available on TI-Nspire CX
	APOSTROPHE,
	/// Not available on TI-Nspire CX
	BAR,
	/// Not available on TI-Nspire CX
	COLON,
	/// Not available on TI-Nspire CX
	COS,
	/// Not available on TI-Nspire CX
	GTHAN,
	/// Not available on TI-Nspire CX
	II,
	/// Not available on TI-Nspire CX
	LTHAN,
	/// Not available on TI-Nspire CX
	QUES,
	/// Not available on TI-Nspire CX
	QUOTE,
	/// Not available on TI-Nspire CX
	SIN,
	/// Not available on TI-Nspire CX
	TAN,
	/// Not available on TI-Nspire CX
	THETA,
}

macro_rules! KEY_ {
	($row:expr, $col:expr) => {
		ndless_sys::t_key {
			row: $row,
			col: $col,
			tpad_row: $row,
			tpad_col: $col,
			tpad_arrow: 0,
		}
	};
}
macro_rules! KEYTPAD_ {
	($row:expr, $col:expr, $tpad_row:expr, $tpad_col:expr) => {
		ndless_sys::t_key {
			row: $row,
			col: $col,
			tpad_row: $tpad_row,
			tpad_col: $tpad_col,
			tpad_arrow: 0,
		}
	};
}
macro_rules! KEYTPAD_ARROW_ {
	($row:expr, $col:expr, $tpad_arrow:expr) => {
		ndless_sys::t_key {
			row: $row,
			col: $col,
			tpad_row: $row,
			tpad_col: $col,
			tpad_arrow: $tpad_arrow,
		}
	};
}

mod raw_keys {
	#![allow(non_camel_case_types)]
	#![allow(non_upper_case_globals)]
	pub const _KEY_DUMMY_ROW: i32 = 0x1C;
	pub const _KEY_DUMMY_COL: i32 = 0x400;
	pub const KEY_NSPIRE_RET: ndless_sys::t_key = KEY_!(0x10, 0x001);
	pub const KEY_NSPIRE_ENTER: ndless_sys::t_key = KEY_!(0x10, 0x002);
	pub const KEY_NSPIRE_SPACE: ndless_sys::t_key = KEYTPAD_!(0x10, 0x004, 0x10, 0x10);
	pub const KEY_NSPIRE_NEGATIVE: ndless_sys::t_key = KEY_!(0x10, 0x008);
	pub const KEY_NSPIRE_Z: ndless_sys::t_key = KEYTPAD_!(0x10, 0x010, 0x10, 0x20);
	pub const KEY_NSPIRE_PERIOD: ndless_sys::t_key = KEYTPAD_!(0x10, 0x20, 0x1A, 0x010);
	pub const KEY_NSPIRE_Y: ndless_sys::t_key = KEY_!(0x10, 0x040);
	pub const KEY_NSPIRE_0: ndless_sys::t_key = KEY_!(0x10, 0x080);
	pub const KEY_NSPIRE_X: ndless_sys::t_key = KEYTPAD_!(0x10, 0x100, 0x12, 0x001);
	pub const KEY_NSPIRE_THETA: ndless_sys::t_key = KEYTPAD_!(0x10, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_COMMA: ndless_sys::t_key = KEYTPAD_!(0x12, 0x001, 0x1E, 0x400);
	pub const KEY_NSPIRE_PLUS: ndless_sys::t_key = KEYTPAD_!(0x12, 0x002, 0x1C, 0x004);
	pub const KEY_NSPIRE_W: ndless_sys::t_key = KEYTPAD_!(0x12, 0x004, 0x12, 0x002);
	pub const KEY_NSPIRE_3: ndless_sys::t_key = KEY_!(0x12, 0x008);
	pub const KEY_NSPIRE_V: ndless_sys::t_key = KEYTPAD_!(0x12, 0x010, 0x12, 0x004);
	pub const KEY_NSPIRE_2: ndless_sys::t_key = KEYTPAD_!(0x12, 0x020, 0x1C, 0x010);
	pub const KEY_NSPIRE_U: ndless_sys::t_key = KEYTPAD_!(0x12, 0x040, 0x12, 0x010);
	pub const KEY_NSPIRE_1: ndless_sys::t_key = KEY_!(0x12, 0x080);
	pub const KEY_NSPIRE_T: ndless_sys::t_key = KEYTPAD_!(0x12, 0x100, 0x12, 0x020);
	pub const KEY_NSPIRE_eEXP: ndless_sys::t_key = KEYTPAD_!(0x12, 0x200, 0x16, 0x200);
	pub const KEY_NSPIRE_PI: ndless_sys::t_key = KEYTPAD_!(0x12, 0x400, 0x12, 0x100);
	pub const KEY_NSPIRE_QUES: ndless_sys::t_key = KEYTPAD_!(0x14, 0x001, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_QUESEXCL: ndless_sys::t_key =
		KEYTPAD_!(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x10, 0x100);
	pub const KEY_NSPIRE_MINUS: ndless_sys::t_key = KEYTPAD_!(0x14, 0x002, 0x1A, 0x004);
	pub const KEY_NSPIRE_S: ndless_sys::t_key = KEYTPAD_!(0x14, 0x004, 0x12, 0x040);
	pub const KEY_NSPIRE_6: ndless_sys::t_key = KEY_!(0x14, 0x008);
	pub const KEY_NSPIRE_R: ndless_sys::t_key = KEYTPAD_!(0x14, 0x010, 0x14, 0x001);
	pub const KEY_NSPIRE_5: ndless_sys::t_key = KEYTPAD_!(0x14, 0x020, 0x1A, 0x040);
	pub const KEY_NSPIRE_Q: ndless_sys::t_key = KEYTPAD_!(0x14, 0x040, 0x14, 0x002);
	pub const KEY_NSPIRE_4: ndless_sys::t_key = KEY_!(0x14, 0x080);
	pub const KEY_NSPIRE_P: ndless_sys::t_key = KEYTPAD_!(0x14, 0x100, 0x14, 0x004);
	pub const KEY_NSPIRE_TENX: ndless_sys::t_key = KEYTPAD_!(0x14, 0x200, 0x12, 0x400);
	pub const KEY_NSPIRE_EE: ndless_sys::t_key = KEYTPAD_!(0x14, 0x400, 0x14, 0x100);
	pub const KEY_NSPIRE_COLON: ndless_sys::t_key = KEYTPAD_!(0x16, 0x001, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_MULTIPLY: ndless_sys::t_key = KEYTPAD_!(0x16, 0x002, 0x18, 0x100);
	pub const KEY_NSPIRE_O: ndless_sys::t_key = KEYTPAD_!(0x16, 0x004, 0x14, 0x010);
	pub const KEY_NSPIRE_9: ndless_sys::t_key = KEY_!(0x16, 0x008);
	pub const KEY_NSPIRE_N: ndless_sys::t_key = KEYTPAD_!(0x16, 0x010, 0x14, 0x020);
	pub const KEY_NSPIRE_8: ndless_sys::t_key = KEYTPAD_!(0x16, 0x020, 0x1C, 0x040);
	pub const KEY_NSPIRE_M: ndless_sys::t_key = KEYTPAD_!(0x16, 0x040, 0x14, 0x040);
	pub const KEY_NSPIRE_7: ndless_sys::t_key = KEY_!(0x16, 0x080);
	pub const KEY_NSPIRE_L: ndless_sys::t_key = KEYTPAD_!(0x16, 0x100, 0x16, 0x001);
	pub const KEY_NSPIRE_SQU: ndless_sys::t_key = KEYTPAD_!(0x16, 0x200, 0x14, 0x200);
	pub const KEY_NSPIRE_II: ndless_sys::t_key = KEYTPAD_!(0x16, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_QUOTE: ndless_sys::t_key = KEYTPAD_!(0x18, 0x001, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_DIVIDE: ndless_sys::t_key = KEYTPAD_!(0x18, 0x002, 0x16, 0x100);
	pub const KEY_NSPIRE_K: ndless_sys::t_key = KEYTPAD_!(0x18, 0x004, 0x16, 0x002);
	pub const KEY_NSPIRE_TAN: ndless_sys::t_key = KEY_!(0x18, 0x008);
	pub const KEY_NSPIRE_J: ndless_sys::t_key = KEYTPAD_!(0x18, 0x010, 0x16, 0x004);
	pub const KEY_NSPIRE_COS: ndless_sys::t_key = KEYTPAD_!(0x18, 0x020, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_I: ndless_sys::t_key = KEYTPAD_!(0x18, 0x040, 0x16, 0x010);
	pub const KEY_NSPIRE_SIN: ndless_sys::t_key = KEYTPAD_!(0x18, 0x080, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_H: ndless_sys::t_key = KEYTPAD_!(0x18, 0x100, 0x16, 0x020);
	pub const KEY_NSPIRE_EXP: ndless_sys::t_key = KEYTPAD_!(0x18, 0x200, 0x18, 0x200);
	pub const KEY_NSPIRE_GTHAN: ndless_sys::t_key = KEYTPAD_!(0x18, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_APOSTROPHE: ndless_sys::t_key = KEY_!(0x1A, 0x001);
	pub const KEY_NSPIRE_CAT: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x002, 0x1A, 0x080);
	pub const KEY_NSPIRE_FRAC: ndless_sys::t_key = KEYTPAD_!(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x1A, 0x100);
	pub const KEY_NSPIRE_G: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x004, 0x16, 0x040);
	pub const KEY_NSPIRE_RP: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x008, 0x1A, 0x008);
	pub const KEY_NSPIRE_F: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x010, 0x18, 0x001);
	pub const KEY_NSPIRE_LP: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x020, 0x1A, 0x020);
	pub const KEY_NSPIRE_E: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x040, 0x18, 0x002);
	pub const KEY_NSPIRE_VAR: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x080, 0x1A, 0x002);
	pub const KEY_NSPIRE_D: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x100, 0x18, 0x004);
	pub const KEY_NSPIRE_DEL: ndless_sys::t_key = KEYTPAD_!(0x1E, 0x100, 0x1A, 0x200);
	pub const KEY_NSPIRE_LTHAN: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x400, _KEY_DUMMY_ROW, _KEY_DUMMY_COL);
	pub const KEY_NSPIRE_FLAG: ndless_sys::t_key = KEY_!(0x1C, 0x001);
	pub const KEY_NSPIRE_CLICK: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1C, 0x002, ndless_sys::tpad_arrow_TPAD_ARROW_CLICK);
	pub const KEY_NSPIRE_C: ndless_sys::t_key = KEYTPAD_!(0x1C, 0x004, 0x18, 0x010);
	pub const KEY_NSPIRE_HOME: ndless_sys::t_key = KEYTPAD_!(0x1C, 0x008, 0x10, 0x200);
	pub const KEY_NSPIRE_B: ndless_sys::t_key = KEYTPAD_!(0x1C, 0x010, 0x18, 0x020);
	pub const KEY_NSPIRE_MENU: ndless_sys::t_key = KEY_!(0x1C, 0x020);
	pub const KEY_NSPIRE_A: ndless_sys::t_key = KEYTPAD_!(0x1C, 0x040, 0x18, 0x040);
	pub const KEY_NSPIRE_ESC: ndless_sys::t_key = KEY_!(0x1C, 0x080);
	pub const KEY_NSPIRE_BAR: ndless_sys::t_key = KEY_!(0x1C, 0x100);
	pub const KEY_NSPIRE_TAB: ndless_sys::t_key = KEY_!(0x1C, 0x200);
	pub const KEY_NSPIRE_EQU: ndless_sys::t_key = KEYTPAD_!(0x1E, 0x400, 0x18, 0x080);
	pub const KEY_NSPIRE_UP: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x001, ndless_sys::tpad_arrow_TPAD_ARROW_UP);
	pub const KEY_NSPIRE_UPRIGHT: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x002, ndless_sys::tpad_arrow_TPAD_ARROW_UPRIGHT);
	pub const KEY_NSPIRE_RIGHT: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x004, ndless_sys::tpad_arrow_TPAD_ARROW_RIGHT);
	pub const KEY_NSPIRE_RIGHTDOWN: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x008, ndless_sys::tpad_arrow_TPAD_ARROW_RIGHTDOWN);
	pub const KEY_NSPIRE_DOWN: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x010, ndless_sys::tpad_arrow_TPAD_ARROW_DOWN);
	pub const KEY_NSPIRE_DOWNLEFT: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x020, ndless_sys::tpad_arrow_TPAD_ARROW_DOWNLEFT);
	pub const KEY_NSPIRE_LEFT: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x040, ndless_sys::tpad_arrow_TPAD_ARROW_LEFT);
	pub const KEY_NSPIRE_LEFTUP: ndless_sys::t_key =
		KEYTPAD_ARROW_!(0x1E, 0x080, ndless_sys::tpad_arrow_TPAD_ARROW_LEFTUP);
	pub const KEY_NSPIRE_SHIFT: ndless_sys::t_key = KEYTPAD_!(0x1A, 0x200, 0x1E, 0x100);
	pub const KEY_NSPIRE_CTRL: ndless_sys::t_key = KEY_!(0x1E, 0x200);
	pub const KEY_NSPIRE_DOC: ndless_sys::t_key = KEYTPAD_!(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x1C, 0x008);
	pub const KEY_NSPIRE_TRIG: ndless_sys::t_key = KEYTPAD_!(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x12, 0x200);
	pub const KEY_NSPIRE_SCRATCHPAD: ndless_sys::t_key =
		KEYTPAD_!(_KEY_DUMMY_ROW, _KEY_DUMMY_COL, 0x1A, 0x400);
}

pub fn get_keys() -> Vec<Key> {
	use raw_keys::*;
	let key_mapping = vec![
		(KEY_NSPIRE_0, Key::KEY_0),
		(KEY_NSPIRE_1, Key::KEY_1),
		(KEY_NSPIRE_2, Key::KEY_2),
		(KEY_NSPIRE_3, Key::KEY_3),
		(KEY_NSPIRE_4, Key::KEY_4),
		(KEY_NSPIRE_5, Key::KEY_5),
		(KEY_NSPIRE_6, Key::KEY_6),
		(KEY_NSPIRE_7, Key::KEY_7),
		(KEY_NSPIRE_8, Key::KEY_8),
		(KEY_NSPIRE_9, Key::KEY_9),
		(KEY_NSPIRE_A, Key::A),
		(KEY_NSPIRE_APOSTROPHE, Key::APOSTROPHE),
		(KEY_NSPIRE_B, Key::B),
		(KEY_NSPIRE_BAR, Key::BAR),
		(KEY_NSPIRE_C, Key::C),
		(KEY_NSPIRE_CAT, Key::CATALOG),
		(KEY_NSPIRE_CLICK, Key::CLICK),
		(KEY_NSPIRE_COLON, Key::COLON),
		(KEY_NSPIRE_COMMA, Key::COMMA),
		(KEY_NSPIRE_COS, Key::COS),
		(KEY_NSPIRE_CTRL, Key::CTRL),
		(KEY_NSPIRE_D, Key::D),
		(KEY_NSPIRE_DEL, Key::DEL),
		(KEY_NSPIRE_DIVIDE, Key::DIVIDE),
		(KEY_NSPIRE_DOC, Key::DOC),
		(KEY_NSPIRE_DOWN, Key::DOWN),
		(KEY_NSPIRE_DOWNLEFT, Key::DOWNLEFT),
		(KEY_NSPIRE_E, Key::E),
		(KEY_NSPIRE_EE, Key::EE),
		(KEY_NSPIRE_ENTER, Key::ENTER),
		(KEY_NSPIRE_EQU, Key::EQUALS),
		(KEY_NSPIRE_ESC, Key::ESC),
		(KEY_NSPIRE_EXP, Key::EXPONENT),
		(KEY_NSPIRE_F, Key::F),
		(KEY_NSPIRE_FLAG, Key::FLAG),
		(KEY_NSPIRE_FRAC, Key::TEMPLATE),
		(KEY_NSPIRE_G, Key::G),
		(KEY_NSPIRE_GTHAN, Key::GTHAN),
		(KEY_NSPIRE_H, Key::H),
		(KEY_NSPIRE_HOME, Key::ON),
		(KEY_NSPIRE_I, Key::I),
		(KEY_NSPIRE_II, Key::II),
		(KEY_NSPIRE_J, Key::J),
		(KEY_NSPIRE_K, Key::K),
		(KEY_NSPIRE_L, Key::L),
		(KEY_NSPIRE_LEFT, Key::LEFT),
		(KEY_NSPIRE_LEFTUP, Key::LEFTUP),
		(KEY_NSPIRE_LP, Key::LEFT_PARENTHESIS),
		(KEY_NSPIRE_LTHAN, Key::LTHAN),
		(KEY_NSPIRE_M, Key::M),
		(KEY_NSPIRE_MENU, Key::MENU),
		(KEY_NSPIRE_MINUS, Key::MINUS),
		(KEY_NSPIRE_MULTIPLY, Key::MULTIPLY),
		(KEY_NSPIRE_N, Key::N),
		(KEY_NSPIRE_NEGATIVE, Key::NEGATIVE),
		(KEY_NSPIRE_O, Key::O),
		(KEY_NSPIRE_P, Key::P),
		(KEY_NSPIRE_PERIOD, Key::PERIOD),
		(KEY_NSPIRE_PI, Key::PI),
		(KEY_NSPIRE_PLUS, Key::PLUS),
		(KEY_NSPIRE_Q, Key::Q),
		(KEY_NSPIRE_QUES, Key::QUES),
		(KEY_NSPIRE_QUESEXCL, Key::QUESTION_EXCLAMATION),
		(KEY_NSPIRE_QUOTE, Key::QUOTE),
		(KEY_NSPIRE_R, Key::R),
		(KEY_NSPIRE_RET, Key::RETURN),
		(KEY_NSPIRE_RIGHT, Key::RIGHT),
		(KEY_NSPIRE_RIGHTDOWN, Key::RIGHTDOWN),
		(KEY_NSPIRE_RP, Key::RIGHT_PARENTHESIS),
		(KEY_NSPIRE_S, Key::S),
		(KEY_NSPIRE_SCRATCHPAD, Key::SCRATCHPAD),
		(KEY_NSPIRE_SHIFT, Key::SHIFT),
		(KEY_NSPIRE_SIN, Key::SIN),
		(KEY_NSPIRE_SPACE, Key::SPACE),
		(KEY_NSPIRE_SQU, Key::SQUARED),
		(KEY_NSPIRE_T, Key::T),
		(KEY_NSPIRE_TAB, Key::TAB),
		(KEY_NSPIRE_TAN, Key::TAN),
		(KEY_NSPIRE_TENX, Key::TEN_EXP),
		(KEY_NSPIRE_THETA, Key::THETA),
		(KEY_NSPIRE_TRIG, Key::TRIG),
		(KEY_NSPIRE_U, Key::U),
		(KEY_NSPIRE_UP, Key::UP),
		(KEY_NSPIRE_UPRIGHT, Key::UPRIGHT),
		(KEY_NSPIRE_V, Key::V),
		(KEY_NSPIRE_VAR, Key::VAR),
		(KEY_NSPIRE_W, Key::W),
		(KEY_NSPIRE_X, Key::X),
		(KEY_NSPIRE_Y, Key::Y),
		(KEY_NSPIRE_Z, Key::Z),
		(KEY_NSPIRE_eEXP, Key::E_EXP),
	];
	let mut keys_pressed = vec![];
	for (raw_key, key) in key_mapping.iter() {
		if unsafe { ndless_sys::isKeyPressed(raw_key) } == 1 {
			keys_pressed.push(*key);
		}
	}
	keys_pressed
}
