//! Only supports sleeping—threads are not supported in ndless.

use core::time::Duration;

/// Puts the current thread to sleep for at least the specified amount of time.
///
/// The thread may sleep longer than the duration specified due to scheduling
/// specifics or platform-dependent functionality. It will never sleep less.
///
/// Note that only millisecond intervals are supported.
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
	unsafe {
		ndless_sys::msleep(dur.as_millis() as u32);
	}
}
