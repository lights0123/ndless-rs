# ndless-macros

Ndless procedural macros for common tasks, such as marking the main function of the program.
It will automatically store the command-line arguments, and mark the function as the entrance
to the program.

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
