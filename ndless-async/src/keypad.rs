//! Keypad
//!
//! Listens for button presses and releases.

use alloc::rc::Rc;
use core::cell::{RefCell, Ref};
use core::{
	mem,
	pin::Pin,
	task::{Context, Poll},
};

use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker};
use ignore_result::Ignore;

use ndless::input::{iter_keys, Key};
use ndless::prelude::*;

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
pub struct KeypadListener {
	queues: RefCell<Vec<Rc<SharedKeyQueue>>>,
	keys: RefCell<Vec<Key>>,
}

impl KeypadListener {
	pub(crate) fn is_empty(&self) -> bool {
		self.queues.borrow().is_empty()
	}
	pub(crate) fn poll(&self) {
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
	/// Each call to
	/// [`stream`][KeypadListener::stream] returns a unique stream, meaning
	/// that calling it from different tasks will allow each task to receive
	/// every event. A buffer of 100 keypress events is allocated. Use
	/// [`stream_with_buffer`][KeypadListener::stream_with_buffer] if you'd
	/// like to specify a custom size.
	pub fn stream(&self) -> KeyStream {
		let mut queues = self.queues.borrow_mut();
		let queue = Rc::new(SharedKeyQueue {
			queue: ArrayQueue::new(100),
			waker: AtomicWaker::new(),
		});
		queues.push(queue.clone());
		KeyStream { queue }
	}
	/// This is the same as[`stream`][KeypadListener::stream], except that it
	/// allows specifying a buffer size other than the default of 100.
	pub fn stream_with_buffer(&self, size: usize) -> KeyStream {
		let mut queues = self.queues.borrow_mut();
		let queue = Rc::new(SharedKeyQueue {
			queue: ArrayQueue::new(size),
			waker: AtomicWaker::new(),
		});
		queues.push(queue.clone());
		KeyStream { queue }
	}
	/// Returns a [`Ref`] to the keys that are currently pressed.
	/// **You must [`drop`] this reference either explicitly or by
	/// ending the current scope before `.await`ing something. The
	/// program will crash otherwise.
	pub fn list_keys(&self) -> Ref<Vec<Key>> {
		self.keys.borrow()
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
}

impl Stream for KeyStream {
	type Item = KeyEvent;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		self.queue.waker.register(cx.waker());
		if let Ok(key) = self.queue.queue.pop() {
			Poll::Ready(Some(key))
		} else {
			Poll::Pending
		}
	}
}
