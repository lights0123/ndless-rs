use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::pin::Pin;

use futures_util::task::{AtomicWaker, Context, Poll};

use ndless::prelude::*;

struct WakerData {
	waker: AtomicWaker,
	done: Cell<bool>,
}

#[derive(Default)]
pub(crate) struct YieldListener {
	wakers: RefCell<Vec<Rc<WakerData>>>,
}

impl YieldListener {
	pub(crate) fn poll(&self) {
		let mut wakers = self.wakers.borrow_mut();
		wakers.retain(|waker| Rc::strong_count(waker) > 1);
		wakers.iter_mut().for_each(|waker| {
			waker.done.set(true);
			waker.waker.wake();
		})
	}
	pub(crate) fn yield_now(&self) -> Yield {
		let waker = Rc::new(WakerData { done: Cell::new(false), waker: AtomicWaker::new() });
		let mut wakers = self.wakers.borrow_mut();
		wakers.push(waker.clone());
		Yield { waker }
	}
}

/// Allows other tasks to run. See
/// [`AsyncListeners::yield_now`][crate::executor::AsyncListeners::yield_now]
/// for more details.
pub struct Yield {
	waker: Rc<WakerData>,
}


impl Future for Yield {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		if self.waker.done.get() {
			Poll::Ready(())
		} else {
			self.waker.waker.register(cx.waker());
			Poll::Pending
		}
	}
}
