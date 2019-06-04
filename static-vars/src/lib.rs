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
