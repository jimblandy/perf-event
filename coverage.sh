#!/usr/bin/env bash

cargo +nightly clean
export RUSTFLAGS="\
       -Zprofile \
       -Ccodegen-units=1 \
       -Copt-level=0 \
       -Clink-dead-code \
       -Coverflow-checks=off \
"
export CARGO_INCREMENTAL=0

cargo +nightly test

grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/

xdg-open target/debug/coverage/index.html
