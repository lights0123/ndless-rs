# ndless-handler
<p>
  <a href="https://crates.io/crates/ndless-handler">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/ndless-handler.svg">
  </a>
  <a href="https://docs.rs/ndless-handler">
    <img alt="Crates.io" src="https://docs.rs/ndless-handler/badge.svg">
  </a>
</p>

Necessary code to use the run Rust code on a TI-Nspire. This crate
provides the required code for allocation and panics. Check out [ndless]
in addition to this crate.

## Features
By default, the features [`eh-personality`], `allocator`, `oom-handler`,
and `panic-handler` are enabled. To disable some (which should not be
required in most cases), pass [`default-features = false`][features] as
an option to this crate, then specify which ones are desired with
`features = ["feature-1", "feature-2"]`. Additionally, the feature
`ctype-ptr` is available but not enabled by default, but should be
enabled when using versions of ndless prior to [this commit][205].

[ndless]: https://crates.io/crates/ndless
[`eh-personality`]: https://www.reddit.com/r/rust/comments/estvau/til_why_the_eh_personality_language_item_is/
[features]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features
[205]: https://github.com/ndless-nspire/Ndless/issues/205
