# ndless-macros
<p>
  <a href="https://crates.io/crates/ndless-macros">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/ndless-macros.svg">
  </a>
  <a href="https://docs.rs/ndless-macros">
    <img alt="Crates.io" src="https://docs.rs/ndless-macros/badge.svg">
  </a>
</p>

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
