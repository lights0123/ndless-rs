# ndless-static-vars
<p>
  <a href="https://crates.io/crates/ndless-static-vars">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/ndless-static-vars.svg">
  </a>
  <a href="https://docs.rs/ndless-static-vars">
    <img alt="Crates.io" src="https://docs.rs/ndless-static-vars/badge.svg">
  </a>
</p>

This crate is designed to store static, program-wide data for Ndless for
the TI-Nspire. This crate exists because Rust will use multiple crates
if their versions are incompatible. This crate should have a constant
major version number, so that programs will always work. You
probably don't want to use this crate directly—check out [ndless]
instead.

[ndless]: https://crates.io/crates/ndless
