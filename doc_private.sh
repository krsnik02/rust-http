#!/bin/sh
RUST_DOC=./wrap_rustdoc.sh cargo doc --no-deps "$@"
