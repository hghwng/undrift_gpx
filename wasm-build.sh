#!/bin/bash -eu

cargo +nightly build --target=wasm32-unknown-unknown -p wasm --release
FILE=target/wasm32-unknown-unknown/release/wasm.wasm
wasm-opt "$FILE" -o "$FILE"
wasm-gc "$FILE"
wasm-bindgen "$FILE" --out-dir wasm/web --no-modules --no-typescript

