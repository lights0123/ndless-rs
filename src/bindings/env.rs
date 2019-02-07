use alloc::vec::IntoIter;

use crate::prelude::*;

pub type Args = IntoIter<String>;

pub fn args() -> Args {
	match unsafe { &crate::ARGUMENTS } {
		None => vec![].into_iter(),
		Some(args) => args.clone().into_iter(),
	}
}
