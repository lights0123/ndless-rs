//! Listens for button presses and releases from the keypad.
//!
//! To get started, [create a `KeypadListener`][KeypadListener::new] and read
//! its documentation.

use alloc::rc::{Rc, Weak};
use core::cell::{Ref, RefCell};
use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker, StreamExt};
use ignore_result::Ignore;
use ndless::input::{iter_keys, Key};
use ndless::prelude::*;
use ndless::timer::{Ticks, TICKS_PER_SECOND};

use crate::timer::TimerListener;

/// The state of the key, either pressed or released.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum KeyState {
	Pressed,
	Released,
}

/// One event representing a key press or release.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct KeyEvent {
	pub key: Key,
	pub state: KeyState,
}

struct SharedKeyQueue {
	queue: ArrayQueue<KeyEvent>,
	waker: AtomicWaker,
}

#[derive(Default)]
struct KeypadListenerInner {
	queues: RefCell<Vec<Rc<SharedKeyQueue>>>,
	keys: RefCell<Vec<Key>>,
}

impl KeypadListenerInner {
	fn poll(&self) {
		let mut queues = self.queues.borrow_mut();
		queues.retain(|queue| Rc::strong_count(queue) > 1);
		if queues.is_empty() {
			return;
		}
		let mut keys = self.keys.borrow_mut();
		let mut retain_i = 0;
		let mut change = false;
		iter_keys().for_each(|key| {
			if let Some((i, _)) = keys.iter().enumerate().find(|(_, other)| key == **other) {
				if i > retain_i {
					let (beginning, end) = keys.split_at_mut(i);
					mem::swap(&mut beginning[retain_i], &mut end[0]);
				}
				retain_i += 1;
			} else {
				change = true;
				keys.push(key);
				queues.iter_mut().for_each(|queue| {
					queue
						.queue
						.push(KeyEvent {
							key,
							state: KeyState::Pressed,
						})
						.ignore()
				});
				if keys.len() > retain_i + 1 {
					let (last, beginning) = keys.split_last_mut().unwrap();
					mem::swap(&mut beginning[retain_i], last);
				}
				retain_i += 1;
			}
		});
		for _ in retain_i..keys.len() {
			change = true;
			let key = keys.pop().unwrap();
			queues.iter_mut().for_each(|queue| {
				queue
					.queue
					.push(KeyEvent {
						key,
						state: KeyState::Released,
					})
					.ignore()
			});
		}
		if change {
			queues.iter_mut().for_each(|queue| queue.waker.wake());
		}
	}
}

/// Polls the keypad.
///
/// ```rust
/// use ndless_async::task::{block_on, AsyncListeners};
/// use ndless_async::keypad::{KeypadListener, KeyState};
/// use ndless_async::StreamExt;
///
/// let listeners = AsyncListeners::new();
/// block_on(&listeners, async {
///     let keypad = KeypadListener::new(&listeners.timer());
///     let mut stream = keypad.stream();
///     while let Some(event) = stream.next().await {
///         println!(
///             "Key {:?} was {}",
///             event.key,
///             if event.state == KeyState::Released {
///                 "released"
///             } else {
///                 "pressed"
///             }
///         );
///     }
/// });
/// ```
pub struct KeypadListener<'a> {
	timer_listener: Option<&'a TimerListener>,
	rate: u32,
	interval: RefCell<Weak<RefCell<dyn Future<Output = ()> + Unpin>>>,
	inner: Rc<KeypadListenerInner>,
}

impl<'a> KeypadListener<'a> {
	/// Creates a new keypad listener that polls the keypad 30 times per second.
	/// Use the `new_with_*` series of functions to change the polling rate. You
	/// may also poll the keypad manually by using
	/// [`new_manually_polled`][KeypadListener::new_manually_polled].
	pub fn new(timer_listener: &'a TimerListener) -> Self {
		Self::new_with_hz(timer_listener, 30)
	}
	/// Creates a new keypad listener that polls the keypad with the specified
	/// number of events per second.
	pub fn new_with_hz(timer_listener: &'a TimerListener, hz: u32) -> Self {
		Self::new_with_ticks(timer_listener, TICKS_PER_SECOND / hz)
	}
	/// Creates a new keypad listener that polls the keypad every `dur`
	/// milliseconds.
	pub fn new_with_ms(timer_listener: &'a TimerListener, dur: u32) -> Self {
		Self::new_with_rate(timer_listener, Duration::from_millis(dur as u64))
	}
	/// Creates a new keypad listener that polls the keypad with the specified
	/// interval.
	pub fn new_with_rate(timer_listener: &'a TimerListener, dur: Duration) -> Self {
		Self::new_with_ticks(timer_listener, dur.as_ticks())
	}
	/// Creates a new keypad listener that polls the keypad every specified
	/// ticks.
	pub fn new_with_ticks(timer_listener: &'a TimerListener, ticks: u32) -> Self {
		Self {
			timer_listener: Some(timer_listener),
			rate: ticks,
			interval: RefCell::new(Weak::<RefCell<futures_util::future::Ready<()>>>::new()),
			inner: Default::default(),
		}
	}
	/// Creates a new keypad listener that isn't automatically polled. You'll
	/// need to use [`poll`][KeypadListener::poll] periodically to poll the
	/// keypad.
	pub fn new_manually_polled() -> Self {
		Self {
			timer_listener: None,
			rate: 0,
			interval: RefCell::new(Weak::<RefCell<futures_util::future::Ready<()>>>::new()),
			inner: Rc::new(Default::default()),
		}
	}
	fn interval(&self) -> Rc<RefCell<dyn Future<Output = ()> + Unpin>> {
		if let Some(interval) = self.interval.borrow().upgrade() {
			return interval;
		}
		let listener = self.inner.clone();
		let interval: Rc<RefCell<dyn Future<Output = ()> + Unpin>> =
			if let Some(timer_listener) = self.timer_listener {
				Rc::new(RefCell::new(
					timer_listener.every_ticks(self.rate).for_each(move |_| {
						listener.poll();
						futures_util::future::ready(())
					}),
				))
			} else {
				Rc::new(RefCell::new(futures_util::future::pending()))
			};
		self.interval.replace(Rc::downgrade(&interval));
		interval
	}
	/// Polls the keypad. You shouldn't have to use this normally.
	pub fn poll(&self) {
		self.inner.poll();
	}
	/// Each call to `stream` returns a unique stream, meaning that calling it
	/// from different tasks will allow each task to receive every event. A
	/// buffer of 100 keypress events is allocated. Use
	/// [`stream_with_buffer`][KeypadListener::stream_with_buffer] to specify a
	/// custom size.
	///
	/// ## Warning
	/// Don't use this function in a loop. You should call `stream` before the
	/// loop, or use a stream combinator such as [`for_each`]. Failure to do so
	/// will result in lost events and less efficient code.
	///
	/// [`for_each`]: https://docs.rs/futures-util/0.3.*/futures_util/stream/trait.StreamExt.html#method.for_each
	pub fn stream(&self) -> KeyStream {
		let mut queues = self.inner.queues.borrow_mut();
		let queue = Rc::new(SharedKeyQueue {
			queue: ArrayQueue::new(100),
			waker: AtomicWaker::new(),
		});
		queues.push(queue.clone());
		KeyStream {
			queue,
			interval: self.interval(),
		}
	}
	/// This is the same as [`stream`][KeypadListener::stream], except that it
	/// allows specifying a buffer size other than the default of 100.
	///
	/// ## Warning
	/// Don't use this function in a loop. You should call `stream_with_buffer`
	/// before the loop, or use a stream combinator such as [`for_each`].
	/// Failure to do so will result in lost events and less efficient code.
	///
	/// [`for_each`]: https://docs.rs/futures-util/0.3.*/futures_util/stream/trait.StreamExt.html#method.for_each
	pub fn stream_with_buffer(&self, size: usize) -> KeyStream {
		let mut queues = self.inner.queues.borrow_mut();
		let queue = Rc::new(SharedKeyQueue {
			queue: ArrayQueue::new(size),
			waker: AtomicWaker::new(),
		});
		queues.push(queue.clone());
		KeyStream {
			queue,
			interval: self.interval(),
		}
	}
	/// Returns a [`Ref`] to the keys that are currently pressed.
	/// **You must [`drop`] this reference either explicitly or by
	/// ending the current scope before `.await`ing something**. The
	/// program will crash otherwise.
	pub fn list_keys(&self) -> Ref<Vec<Key>> {
		self.inner.keys.borrow()
	}
}

/// A stream of [`KeyEvent`]s. Use [`KeypadListener::stream`] to get one.
///
/// # Example
/// ```
/// use ndless_async::StreamExt;
/// use ndless_async::keypad::KeyState::*;
///
/// let mut keypad = listeners.keypad();
/// while let Some(event) = keypad.next().await {
///     println!(
///         "Key {:?} was {}",
///         event.key,
///         if event.state == Released { "released" } else { "pressed" }
///     );
/// }
/// ```
pub struct KeyStream {
	queue: Rc<SharedKeyQueue>,
	interval: Rc<RefCell<dyn Future<Output = ()> + Unpin>>,
}

impl Stream for KeyStream {
	type Item = KeyEvent;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		let mut interval = self.interval.borrow_mut();
		let _ = Pin::new(&mut *interval).poll(cx);
		self.queue.waker.register(cx.waker());
		if let Ok(key) = self.queue.queue.pop() {
			Poll::Ready(Some(key))
		} else {
			Poll::Pending
		}
	}
}
