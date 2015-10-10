#!/bin/sh
cargo build
sudo RUST_LOG=rust_http2=debug ./target/debug/rust-http2
