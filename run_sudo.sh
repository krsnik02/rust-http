#!/bin/sh
cargo build
sudo RUST_LOG=http2=debug ./target/debug/http2
