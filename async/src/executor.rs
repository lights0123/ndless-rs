use alloc::{
	sync::Arc,
	task::Wake
};
use core::future::Future;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, Waker};

use pin_utils::pin_mut;

use ndless::hw::idle;

use crate::keypad::KeypadListener;

pub fn block_on<T>(listeners: &AsyncListeners, task: impl Future<Output = T>) -> T {
	let wake_marker = Arc::new(AtomicBool::new(true));
	let waker = Waker::from(Arc::new(TaskWaker { wake_marker: wake_marker.clone() }));
	let mut context = Context::from_waker(&waker);
	pin_mut!(task);
	loop {
		listeners.keypad.poll();
		if wake_marker.load(Ordering::Relaxed) {
			match task.as_mut().poll(&mut context) {
				Poll::Ready(val) => {
					break val;
				}
				Poll::Pending => {
					wake_marker.store(false, Ordering::Relaxed);
				}
			}
		}
		idle();
	}
}

struct TaskWaker {
	wake_marker: Arc<AtomicBool>,
}

impl TaskWaker {
	fn wake_task(&self) {
		self.wake_marker.store(true, Ordering::Relaxed);
	}
}

impl Wake for TaskWaker {
	fn wake(self: Arc<Self>) {
		self.wake_task();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		self.wake_task();
	}
}

pub struct AsyncListeners {
	keypad: KeypadListener,
}

impl AsyncListeners {
	pub fn new() -> Self {
		AsyncListeners {
			keypad: KeypadListener::new(),
		}
	}
	pub fn keypad(&self) -> &KeypadListener {
		&self.keypad
	}
}
