use alloc::rc::Rc;
use core::{mem, pin::Pin, task::{Context, Poll}};
use core::cell::RefCell;

use crossbeam_queue::ArrayQueue;
use futures_util::{
	stream::{Stream},
	task::AtomicWaker,
};

use ndless::input::{iter_keys, Key};
use ndless::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum KeyState {
	Pressed,
	Released,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct KeyEvent {
	pub key: Key,
	pub state: KeyState,
}

pub struct KeypadListener {
	queue: Rc<ArrayQueue<KeyEvent>>,
	waker: Rc<AtomicWaker>,
	keys: RefCell<Vec<Key>>,
}

impl KeypadListener {
	pub(crate) fn new() -> Self {
		KeypadListener {
			queue: Rc::new(ArrayQueue::new(100)),
			waker: Rc::new(AtomicWaker::new()),
			keys: RefCell::new(vec![]),
		}
	}
	pub(crate) fn poll(&self) {
		let mut retain_i = 0;
		let mut keys = self.keys.borrow_mut();
		iter_keys()
			.for_each(|key| {
				if let Some((i, _)) = keys.iter().enumerate().find(|(_, other)| key == **other) {
					if i > retain_i {
						let (beginning, end) = keys.split_at_mut(i);
						mem::swap(&mut beginning[retain_i], &mut end[0]);
					}
					retain_i += 1;
				} else {
					keys.push(key);
					self.queue.push(KeyEvent { key, state: KeyState::Pressed });
					if keys.len() > retain_i + 1 {
						let (last, beginning) = keys.split_last_mut().unwrap();
						mem::swap(&mut beginning[retain_i], last);
					}
					retain_i += 1;
				}
			});
		for _ in retain_i..keys.len() {
			self.queue.push(KeyEvent { key: keys.pop().unwrap(), state: KeyState::Released });
		}
		if !self.queue.is_empty() { self.waker.wake(); }
	}
	pub fn stream(&self) -> KeyStream {
		KeyStream::new(self.queue.clone(), self.waker.clone())
	}
}

pub struct KeyStream {
	queue: Rc<ArrayQueue<KeyEvent>>,
	waker: Rc<AtomicWaker>,
}

impl KeyStream {
	fn new(queue: Rc<ArrayQueue<KeyEvent>>, waker: Rc<AtomicWaker>) -> Self {
		KeyStream { queue, waker }
	}
}

impl Stream for KeyStream {
	type Item = KeyEvent;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		// fast path
		if let Ok(scancode) = self.queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		self.waker.register(&cx.waker());
		match self.queue.pop() {
			Ok(scancode) => {
				self.waker.take();
				Poll::Ready(Some(scancode))
			}
			_ => Poll::Pending,
		}
	}
}
