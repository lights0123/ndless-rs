//! Keypad
//!
//! Listens for button presses and releases.

use alloc::rc::Rc;
use core::cell::RefCell;
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
pub(crate) struct KeypadListener {
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
	pub(crate) fn stream(&self) -> KeyStream {
		let mut queues = self.queues.borrow_mut();
		let queue = Rc::new(SharedKeyQueue {
			queue: ArrayQueue::new(100),
			waker: AtomicWaker::new(),
		});
		queues.push(queue.clone());
		KeyStream { queue }
	}
}

/// A stream of [`KeyEvent`]s. Use
/// [`AsyncListeners::keypad`][crate::executor::AsyncListeners::keypad] to get one.
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
