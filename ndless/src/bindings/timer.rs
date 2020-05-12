//! Timer
//!
//! Tools for interacting with low-level timers of the nspire.
#![allow(clippy::unreadable_literal)]

use core::ptr::{read_volatile, write_volatile};

use ndless_static_vars::*;

use crate::hw::has_colors;
use crate::time::Duration;

pub const TICKS_PER_SECOND: u32 = 32768;
pub const TICKS_PER_MILLISECOND: u32 = 33;
pub const MICROSECONDS_PER_TICK: u32 = 1000 / TICKS_PER_MILLISECOND;

#[doc(hidden)]
pub fn __init() {
	unsafe {
		if has_colors() {
			let value = 0x900C0004 as *mut u32;
			let control = 0x900C0008 as *mut u32;
			let clock_source = 0x900C0080 as *mut u32;
			write_volatile(clock_source, 0xA);
			write_volatile(control, 0b10000010);
			START_VALUE = read_volatile(value);
		} else {
			let value = 0x900C0000 as *mut u32;
			let control = 0x900C0008 as *mut u32;
			let divider = 0x900C0004 as *mut u32;
			write_volatile(divider, 1);
			write_volatile(control, 0b00001111);
			write_volatile(value, 0);
		}
		init_sleep();
	}
}

/// Returns the number of ticks since the program started, based on
/// a 32768Hz timer (i.e. 32768 ticks per second).
pub fn get_ticks() -> u32 {
	unsafe {
		if has_colors() {
			let value = 0x900C0004 as *mut u32;
			START_VALUE.wrapping_sub(read_volatile(value))
		} else {
			let value = 0x900C0000 as *mut u32;
			TICK_SUM += read_volatile(value);
			write_volatile(value, 0);
			TICK_SUM
		}
	}
}

fn init_sleep() {
	unsafe {
		if has_colors() {
			let control = 0x900D0008 as *mut u32;
			let load = 0x900D0000 as *mut u32;
			let _value = 0x900D0004 as *mut u32;
			ORIG_CONTROL = read_volatile(control);
			ORIG_LOAD = read_volatile(load);
		} else {
			let _timer = 0x900D0000 as *mut u32;
			let control = 0x900D0008 as *mut u32;
			let divider = 0x900D0004 as *mut u32;
			ORIG_DIVIDER = read_volatile(divider);
			ORIG_CONTROL = read_volatile(control);
		}
	}
}

/// Prepares the system for sleep. [`idle`][crate::hw::idle] must be
/// called to actually sleep.
pub fn configure_sleep(ticks: u32) {
	unsafe {
		init_sleep();
		if has_colors() {
			let control = 0x900D0008 as *mut u32;
			let load = 0x900D0000 as *mut u32;
			let _value = 0x900D0004 as *mut u32;
			write_volatile(control, 0);
			write_volatile(control, 0b01100011);
			write_volatile(control, 0b11100011);
			write_volatile(load, ticks);
		} else {
			let timer = 0x900D0000 as *mut u32;
			let control = 0x900D0008 as *mut u32;
			let divider = 0x900D0004 as *mut u32;
			write_volatile(control, 0);
			write_volatile(divider, 1);
			write_volatile(timer, ticks.max(2u32.pow(16) - 1));
		}
	}
}

/// Resets the sleep timer so it may be used normally.
pub fn disable_sleep() {
	unsafe {
		if has_colors() {
			let control = 0x900D0008 as *mut u32;
			let load = 0x900D0000 as *mut u32;
			let _value = 0x900D0004 as *mut u32;
			write_volatile(control, 0);
			write_volatile(control, ORIG_CONTROL & 0b01111111);
			write_volatile(load, ORIG_LOAD);
			write_volatile(control, ORIG_CONTROL);
		} else {
			let timer = 0x900D0000 as *mut u32;
			let control = 0x900D0008 as *mut u32;
			let divider = 0x900D0004 as *mut u32;
			write_volatile(control, ORIG_CONTROL);
			write_volatile(divider, ORIG_DIVIDER);
			write_volatile(timer, 32);
		}
	}
}

/// Detects if the number of ticks has passed yet
pub fn has_time_passed(at_tick: u32) -> bool {
	// https://arduino.stackexchange.com/a/12588/3134
	let half_max = 2u32.pow(31);
	get_ticks().wrapping_sub(at_tick).wrapping_add(half_max) >= half_max
}

/// Utilities to convert standard Rust [`Duration`]s into Nspire ticks
pub trait Ticks {
	fn from_ticks(ticks: u32) -> Self;
	fn as_ticks(&self) -> u32;
}

impl Ticks for Duration {
	fn from_ticks(ticks: u32) -> Self {
		let millis_part = ticks % TICKS_PER_SECOND;
		let micros_part = millis_part % TICKS_PER_MILLISECOND;
		let millis = millis_part / TICKS_PER_MILLISECOND;
		let micros = micros_part * MICROSECONDS_PER_TICK;
		Duration::new(
			(ticks / TICKS_PER_SECOND) as u64,
			millis * 1_000_000 + micros * 1000,
		)
	}

	fn as_ticks(&self) -> u32 {
		self.as_secs() as u32 * TICKS_PER_SECOND
			+ self.subsec_millis() as u32 * TICKS_PER_MILLISECOND
			+ self.subsec_micros() % 1000 / MICROSECONDS_PER_TICK
	}
}
