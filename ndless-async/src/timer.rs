//! Timer
//!
//! Waits for a specific time frame and then completes the `Future`. If
//! no other events are being polled for (i.e. [`keypad`][crate::keypad]),
//! the calculator is put to sleep until the next timer to conserve power.
use alloc::rc::Rc;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;

use futures_util::task::{AtomicWaker, Context, Poll};

use ndless::prelude::*;
use ndless::time::Duration;
use ndless::timer::{configure_sleep, get_ticks};

// https://arduino.stackexchange.com/a/12588/3134
fn has_time_passed(at_tick: u32) -> bool {
	let half_max = 2u32.pow(31);
	get_ticks().wrapping_sub(at_tick).wrapping_add(half_max) >= half_max
}

struct TimerData {
	at_tick: u32,
	waker: AtomicWaker,
}
/// Timer Listener
///
/// Used to create [`Timer`]s, which may be `.await`ed to wait for a specific
/// time. See [`AsyncListeners`][crate::executor::AsyncListeners] to get one.
#[derive(Default)]
pub struct TimerListener {
	timers: RefCell<Vec<Rc<TimerData>>>,
}

impl TimerListener {
	pub(crate) fn poll(&self) {
		let mut timers = self.timers.borrow_mut();
		timers.retain(|timer| Rc::strong_count(timer) > 1);
		timers.iter().for_each(|timer| {
			if has_time_passed(timer.at_tick) {
				timer.waker.wake();
			}
		})
	}
	pub(crate) fn config_sleep(&self) {
		let half_max = 2u32.pow(31);
		let mut timers = self.timers.borrow_mut();
		timers.retain(|timer| Rc::strong_count(timer) > 1);
		if let Some(timer) = timers
			.iter()
			.min_by_key(|timer| timer.at_tick.wrapping_sub(get_ticks()) % half_max)
		{
			configure_sleep(timer.at_tick.wrapping_sub(get_ticks()) % half_max);
		}
	}
	/// Sleeps for the specified number of milliseconds. Problems will occur
	/// when sleeping for more than 2^31/32768 seconds, which is about 18 hours.
	pub fn sleep_ms(&self, ms: u32) -> Timer {
		self.sleep(Duration::from_millis(ms as u64))
	}
	/// Sleeps for the specified [`Duration`]. Problems will occur
	/// when sleeping for more than 2^31/32768 seconds, which is about 18 hours.
	///
	/// This function has a resolution of 30 μs.
	pub fn sleep(&self, dur: Duration) -> Timer {
		// 32768Hz clock, so 33 ticks per ms and 1000 / 33 = 30 μs per tick
		let ticks = dur.as_secs() as u32 * 32768
			+ dur.subsec_millis() as u32 * 33
			+ dur.subsec_micros() % 1000 / 30;
		self.sleep_ticks(ticks)
	}
	/// Sleeps for the specified number of [ticks][ndless::timer::get_ticks].
	/// Problems will occur when sleeping for more than 2^31 ticks,
	/// which is about 18 hours.
	pub fn sleep_ticks(&self, ticks: u32) -> Timer {
		self.sleep_until(get_ticks().wrapping_add(ticks))
	}
	/// Sleeps until the current number of ticks is equal to the parameter.
	/// Problems will occur when sleeping for more than 2^31 ticks in the future,
	/// which is about 18 hours.
	pub fn sleep_until(&self, ticks: u32) -> Timer {
		let timer = Rc::new(TimerData {
			at_tick: ticks,
			waker: AtomicWaker::new(),
		});
		let mut timers = self.timers.borrow_mut();
		timers.push(timer.clone());
		Timer { timer }
	}
}

/// Waits for a specific time. See [TimerListener] for more details.
pub struct Timer {
	timer: Rc<TimerData>,
}

impl Future for Timer {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		if has_time_passed(self.timer.at_tick) {
			Poll::Ready(())
		} else {
			self.timer.waker.register(cx.waker());
			Poll::Pending
		}
	}
}
