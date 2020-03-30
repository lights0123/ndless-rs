#!/usr/bin/env bash

read -r -d '' PREFIX <<"EOF"
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern "C" {
	pub fn abort() -> !;
	pub fn exit(__status: cty::c_int) -> !;
}
EOF

bindgen wrapper.h -o src/lib.rs \
	--raw-line "$PREFIX" \
	--ctypes-prefix=cty \
	--blacklist-item FP_NAN \
	--blacklist-item FP_INFINITE \
	--blacklist-item FP_ZERO \
	--blacklist-item FP_SUBNORMAL \
	--blacklist-item FP_NORMAL \
	--blacklist-function abort \
	--blacklist-function exit \
	--use-core \
	--rust-target nightly \
	-- \
	-I $(dirname $(which nspire-gcc))/../include
