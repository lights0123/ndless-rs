//! Timers, Timeouts, and Intervals
//!
//! Waits for a specific time frame and then completes the `Future`. The
//! calculator is automatically put to sleep until the next timer to conserve
//! power.
//!
//! Check out [`TimerListener`]'s documentation for more.
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use core::fmt;
use core::future::Future;
use core::pin::Pin;

use core::time::Duration;
use futures_util::future::FutureExt;
use futures_util::pin_mut;
use futures_util::stream::Stream;
use futures_util::task::{AtomicWaker, Context, Poll};
use ndless::alloc::fmt::Formatter;
use ndless::prelude::*;
use ndless::timer::{configure_sleep, get_ticks, has_time_passed, Ticks, TICKS_PER_SECOND};

use crate::select;

struct TimerData {
	at_tick: Cell<u32>,
	waker: AtomicWaker,
}

/// Timer Listener
///
/// Used to create [`Timer`]s, which may be `.await`ed to wait for a specific
/// time. See [`AsyncListeners`][crate::task::AsyncListeners] to get one.
#[derive(Default)]
pub struct TimerListener {
	timers: RefCell<Vec<Rc<TimerData>>>,
}

impl TimerListener {
	pub(crate) fn poll(&self) {
		let mut timers = self.timers.borrow_mut();
		timers.retain(|timer| Rc::strong_count(timer) > 1);
		timers.iter().for_each(|timer| {
			if has_time_passed(timer.at_tick.get()) {
				timer.waker.wake();
			}
		})
	}
	pub(crate) fn config_sleep(&self) {
		let half_max = 2u32.pow(31);
		let mut timers = self.timers.borrow_mut();
		timers.retain(|timer| Rc::strong_count(timer) > 1);
		if let Some(time) = timers
			.iter()
			.map(|timer| timer.at_tick.get().wrapping_sub(get_ticks()) % half_max)
			.min()
		{
			configure_sleep(time);
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
		self.sleep_ticks(dur.as_ticks())
	}
	/// Sleeps for the specified number of
	/// [ticks](https://docs.rs/ndless/0.8.*/ndless/timer/fn.get_ticks.html).
	/// Problems will occur when sleeping for more than 2^31 ticks,
	/// which is about 18 hours.
	pub fn sleep_ticks(&self, ticks: u32) -> Timer {
		self.sleep_until(get_ticks().wrapping_add(ticks))
	}
	/// Sleeps until the current number of ticks is equal to the parameter.
	/// Problems will occur when sleeping for more than 2^31 ticks in the
	/// future, which is about 18 hours.
	pub fn sleep_until(&self, ticks: u32) -> Timer {
		let timer = Rc::new(TimerData {
			at_tick: Cell::new(ticks),
			waker: AtomicWaker::new(),
		});
		let mut timers = self.timers.borrow_mut();
		timers.push(timer.clone());
		Timer(timer)
	}
	/// Awaits a future or times out after the specified number of milliseconds.
	/// Problems will occur when sleeping for more than 2^31/32768 seconds,
	/// which is about 18 hours.
	pub async fn timeout_ms<T>(
		&self,
		ms: u32,
		f: impl Future<Output = T>,
	) -> Result<T, TimeoutError> {
		self.timeout(Duration::from_millis(ms as u64), f).await
	}
	/// Awaits a future or times out after the specified [`Duration`]. Problems
	/// will occur when sleeping for more than 2^31/32768 seconds, which is
	/// about 18 hours.
	///
	/// This function has a resolution of 30 μs.
	pub async fn timeout<T>(
		&self,
		dur: Duration,
		f: impl Future<Output = T>,
	) -> Result<T, TimeoutError> {
		self.timeout_ticks(dur.as_ticks(), f).await
	}
	/// Awaits a future or times out after the specified number of
	/// [ticks](https://docs.rs/ndless/0.8.*/ndless/timer/fn.get_ticks.html).
	/// Problems will occur when sleeping for more than 2^31 ticks,
	/// which is about 18 hours.
	pub async fn timeout_ticks<T>(
		&self,
		ticks: u32,
		f: impl Future<Output = T>,
	) -> Result<T, TimeoutError> {
		self.timeout_until(get_ticks().wrapping_add(ticks), f).await
	}
	/// Awaits a future or times out after the current number of ticks is equal
	/// to the parameter. Problems will occur when sleeping for more than 2^31
	/// ticks in the future, which is about 18 hours.
	pub async fn timeout_until<T>(
		&self,
		ticks: u32,
		f: impl Future<Output = T>,
	) -> Result<T, TimeoutError> {
		let f = f.fuse();
		pin_mut!(f);
		select! {
			x = f => Ok(x),
			_ = self.sleep_until(ticks).fuse() => Err(TimeoutError),
		}
	}
	/// Creates a [`Stream`] that triggers with the specified number of events
	/// per second.
	pub fn every_hz(&self, hz: u32) -> Interval {
		self.every_ticks(TICKS_PER_SECOND / hz)
	}
	/// Creates a [`Stream`] that triggers every specified number of
	/// milliseconds. Problems will occur when sleeping for more than 2^31/32768
	/// seconds, which is about 18 hours.
	pub fn every_ms(&self, ms: u32) -> Interval {
		self.every(Duration::from_millis(ms as u64))
	}
	/// Creates a [`Stream`] that triggers every specified [`Duration`].
	/// Problems will occur when sleeping for more than 2^31/32768 seconds,
	/// which is about 18 hours.
	///
	/// This function has a resolution of 30 μs.
	pub fn every(&self, dur: Duration) -> Interval {
		self.every_ticks(dur.as_ticks())
	}
	/// Creates a [`Stream`] that triggers every specified number of
	/// [ticks](https://docs.rs/ndless/0.8.*/ndless/timer/fn.get_ticks.html).
	/// Problems will occur when sleeping for more than 2^31 ticks, which is
	/// about 18 hours.
	pub fn every_ticks(&self, ticks: u32) -> Interval {
		Interval {
			interval: ticks,
			timer: self.sleep_ticks(ticks),
		}
	}
}

/// A timer that keeps re-triggering.
///
/// Use [`TimerListener::every`], [`TimerListener::every_hz`],
/// [`TimerListener::every_ms`], or [`TimerListener::every_ticks`] to get an
/// `Interval`.
///
/// This implements [`Stream`], giving the [`Duration`] of time
/// ago when this *should* have been triggered. If a task takes a lot of time,
/// the [`Stream`] will only produce one event. This is likely what you want for
/// handling events like keypad input, as only one event will trigger after
/// blocking for a while, rather than many events being triggered right after
/// the blocking event.
///
/// ```rust
/// use ndless_async::StreamExt;
/// use ndless_async::task::{block_on, AsyncListeners};
///
/// let listeners = AsyncListeners::new();
/// block_on(&listeners, async {
///     let mut interval = listeners.timer().every_ms(1000);
///     while let Some(d) = interval.next().await {
///         println!("Ping! This event was expected {:?} ago", d);
///     }
/// });
/// ```
pub struct Interval {
	interval: u32,
	timer: Timer,
}

impl Interval {
	/// The interval that this `Interval` triggers
	pub fn interval(&self) -> Duration {
		Duration::from_ticks(self.interval)
	}
	/// The interval, in milliseconds, that this `Interval` triggers
	pub fn interval_ms(&self) -> u32 {
		self.interval().as_ticks()
	}
	/// The interval, in ticks, that this `Interval` triggers
	pub fn interval_ticks(&self) -> u32 {
		self.interval
	}
	/// Reschedules this interval for the specified number of milliseconds.
	/// Problems will occur when sleeping for more than 2^31/32768 seconds,
	/// which is about 18 hours.
	pub fn reschedule_ms(&mut self, ms: u32) {
		self.reschedule(Duration::from_millis(ms as u64))
	}
	/// Reschedules this interval for the specified [`Duration`]. Problems will
	/// occur when sleeping for more than 2^31/32768 seconds, which is about 18
	/// hours.
	///
	/// This function has a resolution of 30 μs.
	pub fn reschedule(&mut self, dur: Duration) {
		self.reschedule_ticks(dur.as_ticks())
	}
	/// Reschedules this interval for the specified number of
	/// [ticks](https://docs.rs/ndless/0.8.*/ndless/timer/fn.get_ticks.html).
	/// Problems will occur when sleeping for more than 2^31 ticks, which is
	/// about 18 hours.
	pub fn reschedule_ticks(&mut self, ticks: u32) {
		self.interval = ticks;
		self.timer.reschedule_ticks(ticks)
	}
}

impl Stream for Interval {
	/// The difference between now and when this event *should* have occurred.
	type Item = Duration;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let res = Pin::new(&mut self.timer).poll(cx);
		match res {
			Poll::Ready(dur) => {
				self.timer.reschedule_ticks(self.interval);
				Poll::Ready(Some(dur))
			}
			Poll::Pending => Poll::Pending,
		}
	}
}

/// Waits for a specific time.
///
/// Use [`TimerListener::sleep`], [`TimerListener::sleep_ms`],
/// [`TimerListener::sleep_ticks`], or [`TimerListener::sleep_until`] to create
/// a `Timer`.
///
/// The timer can be rescheduled with the `reschedule` series of functions. If
/// the original time period has already passed, it will re-trigger after the
/// new time period.
///
/// The calculator is automatically put to sleep until the next timer to
/// conserve power.
///
/// ```rust
/// use ndless_async::task::{block_on, AsyncListeners};
///
/// let listeners = AsyncListeners::new();
/// block_on(&listeners, async {
///     let late_by = listeners.timer().sleep_ms(1000).await;
///     println!("Done sleeping! This event was expected {:?} ago", late_by);
/// });
/// ```
pub struct Timer(Rc<TimerData>);

impl Timer {
	/// Get the tick that this timer should fire at
	pub fn at_tick(&self) -> u32 {
		self.0.at_tick.get()
	}
	/// Reschedules this timer for the specified number of milliseconds.
	/// Problems will occur when sleeping for more than 2^31/32768 seconds,
	/// which is about 18 hours.
	///
	/// If this timer has already triggered, it will trigger again after the
	/// specified delay.
	pub fn reschedule_ms(&self, ms: u32) {
		self.reschedule(Duration::from_millis(ms as u64))
	}
	/// Reschedules this timer for the specified [`Duration`]. Problems will
	/// occur when sleeping for more than 2^31/32768 seconds, which is about 18
	/// hours.
	///
	/// If this timer has already triggered, it will trigger again after the
	/// specified delay.
	///
	/// This function has a resolution of 30 μs.
	pub fn reschedule(&self, dur: Duration) {
		self.reschedule_ticks(dur.as_ticks())
	}
	/// Reschedules this timer for the specified number of
	/// [ticks](https://docs.rs/ndless/0.8.*/ndless/timer/fn.get_ticks.html).
	/// Problems will occur when sleeping for more than 2^31 ticks, which is
	/// about 18 hours.
	///
	/// If this timer has already triggered, it will trigger again after the
	/// specified delay.
	pub fn reschedule_ticks(&self, ticks: u32) {
		self.reschedule_at(get_ticks().wrapping_add(ticks))
	}
	/// Reschedules this timer until the current number of ticks is equal to the
	/// parameter. Problems will occur when sleeping for more than 2^31 ticks in
	/// the future, which is about 18 hours.
	///
	/// If this timer has already triggered, it will trigger again after the
	/// specified delay.
	pub fn reschedule_at(&self, ticks: u32) {
		self.0.at_tick.set(ticks);
	}
}

impl Future for Timer {
	/// The difference between now and when this event *should* have occurred.
	type Output = Duration;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let at_tick = self.at_tick();
		if has_time_passed(at_tick) {
			Poll::Ready(Duration::from_ticks(get_ticks().wrapping_sub(at_tick)))
		} else {
			self.0.waker.register(cx.waker());
			Poll::Pending
		}
	}
}

/// An error returned when a future times out.
///
/// This may occur when using [`TimerListener::timeout`],
/// [`TimerListener::timeout_ms`], [`TimerListener::timeout_ticks`], or
/// [`TimerListener::timeout_until`].
#[derive(Eq, PartialEq, Copy, Clone, Default, Debug, Hash)]
pub struct TimeoutError;

impl fmt::Display for TimeoutError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		"future has timed out".fmt(f)
	}
}
