//! Executor
//!
//! The main ndless executor and reactor. Calling [`block_on`] will wait for the
//! future to complete, then return its result. You'll often want to combine this
//! with [`join`] or [`first`] to run multiple things at once.
//!
//! # Example
//! ```
//! use ndless_async::executor::{AsyncListeners, block_on};
//! use ndless_async::StreamExt;
//! use ndless::input::Key;
//! use ndless::prelude::*;
//!
//! let listeners = AsyncListeners::new();
//! block_on(&listeners, listen(&listeners));
//!
//! async fn listen(listeners: &AsyncListeners) {
//!     let mut keypad = listeners.keypad();
//!     while let Some(event) = keypad.next().await {
//!         println!("{:?}", key);
//!         if event.key == Key::Esc {
//!             break;
//!         }
//!     }
//! }
//! ```

use alloc::{sync::Arc, task::Wake};
use core::future::Future;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, Waker};

use futures_util::pin_mut;

use ndless::hw::idle;
use ndless::timer::disable_sleep;

use crate::keypad::{KeyStream, KeypadListener};
use crate::timer::TimerListener;
use crate::yield_now::{Yield, YieldListener};

/// Spawns a task and blocks until the future resolves, returning its result.
pub fn block_on<T>(listeners: &AsyncListeners, task: impl Future<Output = T>) -> T {
	let wake_marker = Arc::new(AtomicBool::new(true));
	let waker = Waker::from(Arc::new(TaskWaker {
		wake_marker: wake_marker.clone(),
	}));
	let mut context = Context::from_waker(&waker);
	pin_mut!(task);
	let mut task = task;
	loop {
		listeners.keypad.poll();
		listeners.timer.poll();
		listeners.yielder.poll();
		while wake_marker.load(Ordering::Relaxed) {
			match task.as_mut().poll(&mut context) {
				Poll::Ready(val) => {
					disable_sleep();
					return val;
				}
				Poll::Pending => {
					wake_marker.store(false, Ordering::Relaxed);
				}
			}
			listeners.keypad.poll();
			listeners.timer.poll();
			listeners.yielder.poll();
		}
		if listeners.keypad.is_empty() {
			listeners.timer.config_sleep();
		} else {
			disable_sleep();
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

/// Handler for listening to system events.
///
/// Create one with `AsyncListeners::default()` and pass it to [`block_on`], as
/// well as your future. See the [module-level documentation][self] for more.
#[derive(Default)]
pub struct AsyncListeners {
	keypad: KeypadListener,
	timer: TimerListener,
	yielder: YieldListener,
}

impl AsyncListeners {
	/// Returns a stream of keypad events. Each call to
	/// [`keypad`][AsyncListeners::keypad] returns a unique stream, meaning
	/// that calling it from different tasks will allow each task to receive
	/// every event.
	pub fn keypad(&self) -> KeyStream {
		self.keypad.stream()
	}
	/// Returns a [`TimerListener`] instance, which may be used to schedule
	/// timers.
	pub fn timer(&self) -> &TimerListener {
		&self.timer
	}
	/// Allows other tasks to run before coming back to this one. Useful when
	/// doing something computationally intensive, to allow things like keyboard
	/// handlers and timers to run. Note that the calculator will not go to
	/// sleep if this is called in a loop. Use [`timer`][AsyncListeners::timer]
	/// instead if a delay is desired between each iteration. If no other tasks
	/// are scheduled, this task is continued immediately.
	pub fn yield_now(&self) -> Yield {
		self.yielder.yield_now()
	}
}
