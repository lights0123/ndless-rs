#![no_std]
#![feature(alloc)]
extern crate alloc;

use alloc::prelude::*;

pub static mut ARGUMENTS: Option<Vec<String>> = None;
