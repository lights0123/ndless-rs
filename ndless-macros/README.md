# ndless-macros
[![Crates.io](https://img.shields.io/crates/v/ndless-macros.svg)](https://crates.io/crates/ndless-macros)
[![Docs.rs](https://docs.rs/ndless-macros/badge.svg)](https://docs.rs/ndless-macros)

Ndless procedural macros for common tasks, such as marking the main
function of the program. It will automatically store the command-line
arguments, and mark the function as the entrance to the program. You
probably don't want to use this crate directly—check out [ndless]
instead.

[ndless]: https://crates.io/crates/ndless

```rust
#![no_std]
#![no_main]

extern crate ndless_handler;

use ndless::prelude::*;

#[entry]
fn main() {
    // Code
}
```
