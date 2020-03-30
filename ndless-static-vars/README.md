# ndless-static-vars
[![Crates.io](https://img.shields.io/crates/v/ndless-static-vars.svg)](https://crates.io/crates/ndless-static-vars)
[![Docs.rs](https://docs.rs/ndless-static-vars/badge.svg)](https://docs.rs/ndless-static-vars)

This crate is designed to store static, program-wide data for Ndless for
the TI-Nspire. This crate exists because Rust will use multiple crates
if their versions are incompatible. This crate should have a constant
major version number, so that programs will always work. You
probably don't want to use this crate directly—check out [ndless]
instead.

[ndless]: https://crates.io/crates/ndless
