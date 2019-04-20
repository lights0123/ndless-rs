use alloc::vec::IntoIter;

use cstr_core::CStr;

use crate::prelude::*;

pub type Args = IntoIter<String>;

pub fn args() -> Args {
    unsafe { &crate::ARGUMENTS }
        .map(|args| {
            args.iter()
                .map(|arg| {
                    unsafe { CStr::from_ptr(*arg) }
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
        .into_iter()
}
