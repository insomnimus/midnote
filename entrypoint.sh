#!/bin/sh

set -e

cd /source

echo "::debug::checking code with clippy"
cargo clippy -- -D warnings

echo "::debug::starting build"
cargo build --locked --release
