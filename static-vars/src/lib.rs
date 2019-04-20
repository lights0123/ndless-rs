#![no_std]

pub enum ProgramState {
	Normal,
	Resident,
}

pub static mut PROGRAM_STATE: ProgramState = ProgramState::Normal;

pub static mut ARGUMENTS: Option<&[*const cty::c_char]> = None;
