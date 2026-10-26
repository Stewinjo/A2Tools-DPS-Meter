#!/bin/sh
# Channel named by rust-toolchain.toml, for CI steps that install Rust.
set -eu
grep -m1 '^channel' rust-toolchain.toml | cut -d'"' -f2
