#![feature(wake_trait)]
#![no_std]
extern crate alloc;

pub use futures_util::{join, select_biased, try_join, FutureExt, StreamExt};

pub use yield_now::Yield;

pub mod executor;
pub mod keypad;
pub mod mpsc;
pub mod timer;
mod yield_now;
/// Polls for the first future to complete, and then cancels the remaining ones.
/// If you care about the return value, use [`select_biased`]. This macro must
/// be used in an `async` context, such as an `async fn` or `async { }` block.
///
/// # Examples
/// The call to [`block_on`][executor::block_on] completes after 5 seconds or
/// when the escape key is pressed, whichever comes first.
/// ```
/// use ndless_async::executor::{AsyncListeners, block_on};
/// use ndless_async::StreamExt;
/// use ndless::input::Key;
///
/// let listeners = AsyncListeners::new();
/// block_on(&listeners, async { first!(timeout(&listeners), listen_for_esc(&listeners)) });
///
///
/// async fn timeout(listeners: &AsyncListeners) {
///     listeners.timer().sleep_ms(5000).await;
/// }
///
/// async fn listen_for_esc(listeners: &AsyncListeners) {
///     let mut keypad = listeners.keypad();
///     while let Some(event) = keypad.next().await {
///         if event.key == Key::Esc {
///             break;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! first {
	($( $arg:expr ),*) => (
		$crate::select_biased!($(_ = $crate::FutureExt::fuse($arg) => (),)*)
	)
}
