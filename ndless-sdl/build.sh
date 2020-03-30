#!/usr/bin/env bash

bindgen wrapper.h -o src/gfx/primitives.rs \
	--ctypes-prefix=cty \
	--use-core \
	--rust-target nightly \
	--no-layout-tests \
	-- \
-I $(dirname $(which nspire-gcc))/../include/SDL\
-D_TINSPIRE
