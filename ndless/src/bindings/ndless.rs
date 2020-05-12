//! # Various ndless-related functions
//! This module contains functions that configure miscellaneous settings used in ndless.

pub fn assert_ndless_rev(required_version: u32) {
	unsafe { ndless_sys::assert_ndless_rev(required_version) }
}

pub fn is_startup() -> bool {
	unsafe { ndless_sys::nl_isstartup() > 0 }
}

/// Trigger a breakpoint. If no debugger is connected (i.e. a physical calculator), the calculator
/// will reset. This function will do nothing if compiled in release mode, allowing you to leave
/// this in when compiling for an actual calculator.
pub fn bkpt() {
	if cfg!(debug_assertions) {
		unsafe { llvm_asm!(".long 0xE1212374") }
	}
}

/// See
/// [Hackspire](https://hackspire.org/index.php/Ndless_features_and_limitations#Resident_programs)
pub fn set_resident() {
	unsafe {
		if ndless_static_vars::PROGRAM_STATE == ndless_static_vars::ProgramState::Normal {
			ndless_sys::nl_set_resident();
			ndless_static_vars::ARGUMENTS = None;
			ndless_static_vars::PROGRAM_STATE = ndless_static_vars::ProgramState::Resident;
		}
	}
}

/// Must be called at the end of a program that creates or deletes files,
/// to update the OS document browser.
pub fn refresh_documents() {
	unsafe { ndless_sys::refresh_osscr() }
}

/// return true if a third-party Launcher was used to boot the OS, such as nLaunch/nLaunchy
pub fn third_party_loader() -> bool {
	unsafe { ndless_sys::nl_loaded_by_3rd_party_loader() > 0 }
}
