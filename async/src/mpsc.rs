use alloc::rc::Rc;
use core::cell::Cell;
use core::pin::Pin;

use crossbeam_queue::{ArrayQueue, PushError};
use futures_util::stream::Stream;
use futures_util::task::{AtomicWaker, Context, Poll};

pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
	let queue = Rc::new(ArrayQueue::new(buffer));
	let waker = Rc::new(AtomicWaker::new());
	let ended = Rc::new(Cell::new(false));
	(
		Sender { queue: queue.clone(), waker: waker.clone(), ended: ended.clone() },
		Receiver { queue, waker, ended }
	)
}

#[derive(Clone)]
pub struct Sender<T> {
	queue: Rc<ArrayQueue<T>>,
	waker: Rc<AtomicWaker>,
	ended: Rc<Cell<bool>>,
}

impl<T> Sender<T> {
	pub fn send(&self, data: T) -> Result<(), PushError<T>> {
		self.waker.wake();
		self.queue.push(data)
	}
	pub fn capacity(&self) -> usize {
		self.queue.capacity()
	}
	pub fn is_empty(&self) -> bool {
		self.queue.is_empty()
	}
	pub fn is_full(&self) -> bool {
		self.queue.is_full()
	}
	pub fn len(&self) -> usize {
		self.queue.len()
	}
}

impl<T> Drop for Sender<T> {
	fn drop(&mut self) {
		if Rc::strong_count(&self.ended) <= 2 {
			self.ended.set(true)
		}
	}
}

pub struct Receiver<T> {
	queue: Rc<ArrayQueue<T>>,
	waker: Rc<AtomicWaker>,
	ended: Rc<Cell<bool>>,
}

impl<T> Stream for Receiver<T> {
	type Item = T;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		self.waker.register(&cx.waker());
		match self.queue.pop() {
			Ok(data) => {
				self.waker.take();
				Poll::Ready(Some(data))
			}
			_ => {
				if self.ended.get() {
					Poll::Ready(None)
				} else {
					Poll::Pending
				}
			},
		}
	}
}
