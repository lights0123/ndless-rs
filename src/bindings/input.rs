use crate::prelude::*;

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Key {
	NONE,
	KEY_NSPIRE_0,
	KEY_NSPIRE_1,
	KEY_NSPIRE_2,
	KEY_NSPIRE_3,
	KEY_NSPIRE_4,
	KEY_NSPIRE_5,
	KEY_NSPIRE_6,
	KEY_NSPIRE_7,
	KEY_NSPIRE_8,
	KEY_NSPIRE_9,
	KEY_NSPIRE_A,
	KEY_NSPIRE_APOSTROPHE,
	KEY_NSPIRE_B,
	KEY_NSPIRE_BAR,
	KEY_NSPIRE_C,
	KEY_NSPIRE_CAT,
	KEY_NSPIRE_CLICK,
	KEY_NSPIRE_COLON,
	KEY_NSPIRE_COMMA,
	KEY_NSPIRE_COS,
	KEY_NSPIRE_CTRL,
	KEY_NSPIRE_D,
	KEY_NSPIRE_DEL,
	KEY_NSPIRE_DIVIDE,
	KEY_NSPIRE_DOC,
	KEY_NSPIRE_DOWN,
	KEY_NSPIRE_DOWNLEFT,
	KEY_NSPIRE_E,
	KEY_NSPIRE_EE,
	KEY_NSPIRE_ENTER,
	KEY_NSPIRE_EQU,
	KEY_NSPIRE_ESC,
	KEY_NSPIRE_EXP,
	KEY_NSPIRE_F,
	KEY_NSPIRE_FLAG,
	KEY_NSPIRE_FRAC,
	KEY_NSPIRE_G,
	KEY_NSPIRE_GTHAN,
	KEY_NSPIRE_H,
	KEY_NSPIRE_HOME,
	KEY_NSPIRE_I,
	KEY_NSPIRE_II,
	KEY_NSPIRE_J,
	KEY_NSPIRE_K,
	KEY_NSPIRE_L,
	KEY_NSPIRE_LEFT,
	KEY_NSPIRE_LEFTUP,
	KEY_NSPIRE_LP,
	KEY_NSPIRE_LTHAN,
	KEY_NSPIRE_M,
	KEY_NSPIRE_MENU,
	KEY_NSPIRE_MINUS,
	KEY_NSPIRE_MULTIPLY,
	KEY_NSPIRE_N,
	KEY_NSPIRE_NEGATIVE,
	KEY_NSPIRE_O,
	KEY_NSPIRE_P,
	KEY_NSPIRE_PERIOD,
	KEY_NSPIRE_PI,
	KEY_NSPIRE_PLUS,
	KEY_NSPIRE_Q,
	KEY_NSPIRE_QUES,
	KEY_NSPIRE_QUESEXCL,
	KEY_NSPIRE_QUOTE,
	KEY_NSPIRE_R,
	KEY_NSPIRE_RET,
	KEY_NSPIRE_RIGHT,
	KEY_NSPIRE_RIGHTDOWN,
	KEY_NSPIRE_RP,
	KEY_NSPIRE_S,
	KEY_NSPIRE_SCRATCHPAD,
	KEY_NSPIRE_SHIFT,
	KEY_NSPIRE_SIN,
	KEY_NSPIRE_SPACE,
	KEY_NSPIRE_SQU,
	KEY_NSPIRE_T,
	KEY_NSPIRE_TAB,
	KEY_NSPIRE_TAN,
	KEY_NSPIRE_TENX,
	KEY_NSPIRE_THETA,
	KEY_NSPIRE_TRIG,
	KEY_NSPIRE_U,
	KEY_NSPIRE_UP,
	KEY_NSPIRE_UPRIGHT,
	KEY_NSPIRE_V,
	KEY_NSPIRE_VAR,
	KEY_NSPIRE_W,
	KEY_NSPIRE_X,
	KEY_NSPIRE_Y,
	KEY_NSPIRE_Z,
	KEY_NSPIRE_eEXP,
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
		(KEY_NSPIRE_0, Key::KEY_NSPIRE_0),
		(KEY_NSPIRE_1, Key::KEY_NSPIRE_1),
		(KEY_NSPIRE_2, Key::KEY_NSPIRE_2),
		(KEY_NSPIRE_3, Key::KEY_NSPIRE_3),
		(KEY_NSPIRE_4, Key::KEY_NSPIRE_4),
		(KEY_NSPIRE_5, Key::KEY_NSPIRE_5),
		(KEY_NSPIRE_6, Key::KEY_NSPIRE_6),
		(KEY_NSPIRE_7, Key::KEY_NSPIRE_7),
		(KEY_NSPIRE_8, Key::KEY_NSPIRE_8),
		(KEY_NSPIRE_9, Key::KEY_NSPIRE_9),
		(KEY_NSPIRE_A, Key::KEY_NSPIRE_A),
		(KEY_NSPIRE_APOSTROPHE, Key::KEY_NSPIRE_APOSTROPHE),
		(KEY_NSPIRE_B, Key::KEY_NSPIRE_B),
		(KEY_NSPIRE_BAR, Key::KEY_NSPIRE_BAR),
		(KEY_NSPIRE_C, Key::KEY_NSPIRE_C),
		(KEY_NSPIRE_CAT, Key::KEY_NSPIRE_CAT),
		(KEY_NSPIRE_CLICK, Key::KEY_NSPIRE_CLICK),
		(KEY_NSPIRE_COLON, Key::KEY_NSPIRE_COLON),
		(KEY_NSPIRE_COMMA, Key::KEY_NSPIRE_COMMA),
		(KEY_NSPIRE_COS, Key::KEY_NSPIRE_COS),
		(KEY_NSPIRE_CTRL, Key::KEY_NSPIRE_CTRL),
		(KEY_NSPIRE_D, Key::KEY_NSPIRE_D),
		(KEY_NSPIRE_DEL, Key::KEY_NSPIRE_DEL),
		(KEY_NSPIRE_DIVIDE, Key::KEY_NSPIRE_DIVIDE),
		(KEY_NSPIRE_DOC, Key::KEY_NSPIRE_DOC),
		(KEY_NSPIRE_DOWN, Key::KEY_NSPIRE_DOWN),
		(KEY_NSPIRE_DOWNLEFT, Key::KEY_NSPIRE_DOWNLEFT),
		(KEY_NSPIRE_E, Key::KEY_NSPIRE_E),
		(KEY_NSPIRE_EE, Key::KEY_NSPIRE_EE),
		(KEY_NSPIRE_ENTER, Key::KEY_NSPIRE_ENTER),
		(KEY_NSPIRE_EQU, Key::KEY_NSPIRE_EQU),
		(KEY_NSPIRE_ESC, Key::KEY_NSPIRE_ESC),
		(KEY_NSPIRE_EXP, Key::KEY_NSPIRE_EXP),
		(KEY_NSPIRE_F, Key::KEY_NSPIRE_F),
		(KEY_NSPIRE_FLAG, Key::KEY_NSPIRE_FLAG),
		(KEY_NSPIRE_FRAC, Key::KEY_NSPIRE_FRAC),
		(KEY_NSPIRE_G, Key::KEY_NSPIRE_G),
		(KEY_NSPIRE_GTHAN, Key::KEY_NSPIRE_GTHAN),
		(KEY_NSPIRE_H, Key::KEY_NSPIRE_H),
		(KEY_NSPIRE_HOME, Key::KEY_NSPIRE_HOME),
		(KEY_NSPIRE_I, Key::KEY_NSPIRE_I),
		(KEY_NSPIRE_II, Key::KEY_NSPIRE_II),
		(KEY_NSPIRE_J, Key::KEY_NSPIRE_J),
		(KEY_NSPIRE_K, Key::KEY_NSPIRE_K),
		(KEY_NSPIRE_L, Key::KEY_NSPIRE_L),
		(KEY_NSPIRE_LEFT, Key::KEY_NSPIRE_LEFT),
		(KEY_NSPIRE_LEFTUP, Key::KEY_NSPIRE_LEFTUP),
		(KEY_NSPIRE_LP, Key::KEY_NSPIRE_LP),
		(KEY_NSPIRE_LTHAN, Key::KEY_NSPIRE_LTHAN),
		(KEY_NSPIRE_M, Key::KEY_NSPIRE_M),
		(KEY_NSPIRE_MENU, Key::KEY_NSPIRE_MENU),
		(KEY_NSPIRE_MINUS, Key::KEY_NSPIRE_MINUS),
		(KEY_NSPIRE_MULTIPLY, Key::KEY_NSPIRE_MULTIPLY),
		(KEY_NSPIRE_N, Key::KEY_NSPIRE_N),
		(KEY_NSPIRE_NEGATIVE, Key::KEY_NSPIRE_NEGATIVE),
		(KEY_NSPIRE_O, Key::KEY_NSPIRE_O),
		(KEY_NSPIRE_P, Key::KEY_NSPIRE_P),
		(KEY_NSPIRE_PERIOD, Key::KEY_NSPIRE_PERIOD),
		(KEY_NSPIRE_PI, Key::KEY_NSPIRE_PI),
		(KEY_NSPIRE_PLUS, Key::KEY_NSPIRE_PLUS),
		(KEY_NSPIRE_Q, Key::KEY_NSPIRE_Q),
		(KEY_NSPIRE_QUES, Key::KEY_NSPIRE_QUES),
		(KEY_NSPIRE_QUESEXCL, Key::KEY_NSPIRE_QUESEXCL),
		(KEY_NSPIRE_QUOTE, Key::KEY_NSPIRE_QUOTE),
		(KEY_NSPIRE_R, Key::KEY_NSPIRE_R),
		(KEY_NSPIRE_RET, Key::KEY_NSPIRE_RET),
		(KEY_NSPIRE_RIGHT, Key::KEY_NSPIRE_RIGHT),
		(KEY_NSPIRE_RIGHTDOWN, Key::KEY_NSPIRE_RIGHTDOWN),
		(KEY_NSPIRE_RP, Key::KEY_NSPIRE_RP),
		(KEY_NSPIRE_S, Key::KEY_NSPIRE_S),
		(KEY_NSPIRE_SCRATCHPAD, Key::KEY_NSPIRE_SCRATCHPAD),
		(KEY_NSPIRE_SHIFT, Key::KEY_NSPIRE_SHIFT),
		(KEY_NSPIRE_SIN, Key::KEY_NSPIRE_SIN),
		(KEY_NSPIRE_SPACE, Key::KEY_NSPIRE_SPACE),
		(KEY_NSPIRE_SQU, Key::KEY_NSPIRE_SQU),
		(KEY_NSPIRE_T, Key::KEY_NSPIRE_T),
		(KEY_NSPIRE_TAB, Key::KEY_NSPIRE_TAB),
		(KEY_NSPIRE_TAN, Key::KEY_NSPIRE_TAN),
		(KEY_NSPIRE_TENX, Key::KEY_NSPIRE_TENX),
		(KEY_NSPIRE_THETA, Key::KEY_NSPIRE_THETA),
		(KEY_NSPIRE_TRIG, Key::KEY_NSPIRE_TRIG),
		(KEY_NSPIRE_U, Key::KEY_NSPIRE_U),
		(KEY_NSPIRE_UP, Key::KEY_NSPIRE_UP),
		(KEY_NSPIRE_UPRIGHT, Key::KEY_NSPIRE_UPRIGHT),
		(KEY_NSPIRE_V, Key::KEY_NSPIRE_V),
		(KEY_NSPIRE_VAR, Key::KEY_NSPIRE_VAR),
		(KEY_NSPIRE_W, Key::KEY_NSPIRE_W),
		(KEY_NSPIRE_X, Key::KEY_NSPIRE_X),
		(KEY_NSPIRE_Y, Key::KEY_NSPIRE_Y),
		(KEY_NSPIRE_Z, Key::KEY_NSPIRE_Z),
		(KEY_NSPIRE_eEXP, Key::KEY_NSPIRE_eEXP),
	];
	let mut keys_pressed = vec![];
	for (raw_key, key) in key_mapping.iter() {
		if unsafe { ndless_sys::isKeyPressed(raw_key) } == 1 {
			keys_pressed.push(*key);
		}
	}
	keys_pressed
}
