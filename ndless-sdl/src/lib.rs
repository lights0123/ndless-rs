#![no_std]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::enum_clike_unportable_variant)]
//! # SDL bindings for Ndless
//! Get started with:
//! ```
//! ndless_sdl::init(&[ndless_sdl::InitFlag::Video]);
//! let screen = match ndless_sdl::video::set_video_mode(320, 240, 16,
//!                                                      &[SurfaceFlag::SWSurface],
//!                                                      &[VideoFlag::NoFrame]) {
//!     Ok(screen) => screen,
//!     Err(err) => panic!("failed to set video mode: {}", err)
//! };
//! loop {
//!     screen.fill_rect(Some(ndless_sdl::Rect {
//!          x: 0,
//!          y: 0,
//!          w: 320,
//!          h: 240,
//!     }), ndless_sdl::video::RGB(142, 120, 255));
//! }
//! ndless_sdl::quit();
//! ```
//!
//! It is not recommended to use the input methods from this crate. Rather, use the ones built
//! into the ndless crate.
extern crate num;

pub use sdl::*;

pub mod event;
pub mod gl;
pub mod keysym;
pub mod mouse;
pub mod nsdl;
pub mod video;
pub mod wm;

pub mod image;
pub mod sdl;
pub mod text;

pub mod gfx;
