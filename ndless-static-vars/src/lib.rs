#![no_std]

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum ProgramState {
	Normal,
	Resident,
}

impl Default for ProgramState {
	fn default() -> Self {
		ProgramState::Normal
	}
}

pub static mut PROGRAM_STATE: ProgramState = ProgramState::Normal;

pub static mut ARGUMENTS: Option<&[*const cty::c_char]> = None;

pub static mut ORIG_DIVIDER: u32 = 0;
pub static mut ORIG_CONTROL: u32 = 0;
pub static mut ORIG_LOAD: u32 = 0;

pub static mut TICK_SUM: u32 = 0;
pub static mut START_VALUE: u32 = 0;
