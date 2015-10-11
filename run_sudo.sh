#!/bin/sh
cargo build
sudo RUST_LOG=http=debug ./target/debug/http-server
