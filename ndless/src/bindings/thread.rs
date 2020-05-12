//! Only supports sleeping—threads are not supported in ndless.

use core::time::Duration;

use crate::hw::idle;
use crate::timer::{configure_sleep, disable_sleep, get_ticks, has_time_passed, Ticks};

/// Puts the current thread to sleep for at least the specified amount of time.
///
/// The thread may sleep longer than the duration specified due to scheduling
/// specifics or platform-dependent functionality. It will never sleep less.
///
/// Problems will occur when sleeping for more than 2^31/32768 seconds, which
/// is about 18 hours.
///
/// This function has a resolution of 30 μs.
///
/// # Examples
///
/// ```no_run
/// use core::time;
/// use ndless::thread;
///
/// let ten_millis = time::Duration::from_millis(10);
///
/// thread::sleep(ten_millis);
/// ```
pub fn sleep(dur: Duration) {
	let ticks = dur.as_ticks();
	let wanted_time = get_ticks().wrapping_add(ticks);
	configure_sleep(ticks);
	while !has_time_passed(wanted_time) {
		idle();
	}
	disable_sleep();
}
