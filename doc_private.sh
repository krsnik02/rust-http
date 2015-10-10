#!/bin/sh
RUSTDOC=./wrap_rustdoc.sh cargo doc --no-deps "$@"
