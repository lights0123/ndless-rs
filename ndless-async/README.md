# ndless-async
[![Crates.io](https://img.shields.io/crates/v/ndless-async.svg)](https://crates.io/crates/ndless-async)
[![Docs.rs](https://docs.rs/ndless-async/badge.svg)](https://docs.rs/ndless-async)

Ndless-specific integration with async/await for the TI-Nspire

This crate provides an executor, reactor, and utilities to use Rust's
`async` capabilities with the TI Nspire's timer and keypad. Note that
normally `async` functions are used for I/O. However, as far as I'm aware,
the TI-Nspire's OS has no support for asynchronous I/O of any sort. However,
this still provides helpful utilities for doing multiple things at once,
such as waiting for a key with a timeout.

You'll first need to create an instance of
`AsyncListeners` with `AsyncListeners::default()`.
This allows you to receive events from the Nspire's timer. From there, you
can pass it into `task::block_on`, along with a `Future` of your choice.

# Helpful resources
Check out the [Rust Async Book](https://rust-lang.github.io/async-book/).
This has useful instructions about asynchronous programming. Although it is
mostly written for usage with a full operating system, everything applies
here except chapters 1.4, 6.3, 8, and 10.

`futures_util` has many useful utility functions. Add it to your project
by adding the following to your Cargo.toml:

```toml
futures-util = { version = "0.3.5", default-features = false, features = ["alloc", "async-await-macro"] }
```

You may find its
[`FuturesUnordered`](https://docs.rs/futures-util/0.3.*/futures_util/stream/futures_unordered/struct.FuturesUnordered.html)
to be of help for scheduling multiple tasks. Although
macros like `join` and `first` can be helpful, they aren't as efficient
and flexible as it.

The macros `join`, `select`, `try_join`, and traits `FutureExt` &
`StreamExt` are re-exported from it, so if that's all you need, you don't
need to depend on it directly.

# Example

```rust
use futures_util::future;
use ndless_async::task::{block_on, AsyncListeners};
use ndless_async::{first, StreamExt};
use ndless_async::keypad::KeypadListener;
use ndless::input::Key;

fn main() {
    let listeners = AsyncListeners::new();
    let keypad = KeypadListener::new(&listeners.timer());
    block_on(&listeners, async {
        let _ = listeners.timer().timeout_ms(5000, do_stuff(&keypad)).await;
        listeners.timer().sleep_ms(2000).await;
        first!(do_other_stuff(&listeners), wait_for_esc(&keypad));
    });
}

async fn wait_for_esc(keypad: &KeypadListener<'_>) {
    keypad
        .stream()
        .filter(|key| future::ready(key.key == Key::Esc))
        .next()
        .await;
}

async fn do_other_stuff(listeners: &AsyncListeners) {
    loop {
        listeners.timer().sleep_ms(1000).await;
        println!("1s!");
    }
}

async fn do_stuff(listeners: &KeypadListener<'_>) {
    use ndless_async::keypad::KeyState::*;
    let mut keypad = listeners.stream();
    while let Some(event) = keypad.next().await {
        println!(
            "Key {:?} was {}",
            event.key,
            if event.state == Released {
                "released"
            } else {
                "pressed"
            }
        );
        print!("Keys currently pressed: ");
        listeners
            .list_keys()
            .iter()
            .for_each(|key| print!("{:?} ", key));
        println!();
        if event.key == Key::Esc { break; }
    }
}
```

Check out [ndless] in addition to this crate.

[ndless]: https://crates.io/crates/ndless
