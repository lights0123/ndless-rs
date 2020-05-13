//! A multi-producer, single-consumer queue for sending values across
//! asynchronous tasks.
//!
//! Similarly to the `std`, channel creation provides [`Receiver`] and
//! [`Sender`] handles. [`Receiver`] implements [`Stream`] and allows a task to
//! read values out of the channel. If there is no message to read from the
//! channel, the current task will be notified when a new value is sent.
//! [`Sender`] allows a task to send messages into
//! the channel. If the channel is at capacity, the send will be rejected.
//!
//! # Disconnection
//!
//! When all [`Sender`] handles have been dropped, it is no longer
//! possible to send values into the channel. This is considered the termination
//! event of the stream. As such, [`Receiver::poll_next`][Stream::poll_next]
//! will return `Ok(Ready(None))`.

use alloc::rc::Rc;
use core::pin::Pin;

use crossbeam_queue::{ArrayQueue, PushError};
use futures_util::stream::Stream;
use futures_util::task::{AtomicWaker, Context, Poll};

struct Queue<T> {
	queue: ArrayQueue<T>,
	waker: AtomicWaker,
}

/// Creates a bounded mpsc channel for communicating between asynchronous tasks.
///
/// Being bounded, this channel provides backpressure to ensure that the sender
/// outpaces the receiver by only a limited amount. The channel's capacity is
/// equal to `buffer`. In other words, there are
/// `buffer` "first come, first serve" slots available to all senders.
///
/// The [`Receiver`](Receiver) returned implements the
/// [`Stream`](Stream) trait, while [`Sender`](Sender)
/// has its own method, [`send`][Sender::send].
pub fn channel<T>(buffer: usize) -> (Sender<T>, Receiver<T>) {
	let queue = Rc::new(Queue {
		queue: ArrayQueue::new(buffer),
		waker: Default::default(),
	});
	(
		Sender {
			queue: queue.clone(),
		},
		Receiver { queue },
	)
}

/// The transmission end of a bounded mpsc channel.
///
/// This value is created by the [`channel`] function.
#[derive(Clone)]
pub struct Sender<T> {
	queue: Rc<Queue<T>>,
}

impl<T> Sender<T> {
	/// Sends data across to the receiver.
	pub fn send(&self, data: T) -> Result<(), PushError<T>> {
		self.queue.waker.wake();
		self.queue.queue.push(data)
	}
	pub fn capacity(&self) -> usize {
		self.queue.queue.capacity()
	}
	pub fn is_empty(&self) -> bool {
		self.queue.queue.is_empty()
	}
	pub fn is_full(&self) -> bool {
		self.queue.queue.is_full()
	}
	pub fn len(&self) -> usize {
		self.queue.queue.len()
	}
}

/// The receiving end of a bounded mpsc channel.
///
/// This value is created by the [`channel`] function.
pub struct Receiver<T> {
	queue: Rc<Queue<T>>,
}

impl<T> Stream for Receiver<T> {
	type Item = T;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		self.queue.waker.register(&cx.waker());
		match self.queue.queue.pop() {
			Ok(data) => Poll::Ready(Some(data)),
			_ => {
				if Rc::strong_count(&self.queue) < 2 {
					Poll::Ready(None)
				} else {
					Poll::Pending
				}
			}
		}
	}
}
