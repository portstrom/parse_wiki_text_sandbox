#!/bin/sh
set -e
export RUST_BACKTRACE=1
cargo +nightly build --release --target wasm32-unknown-unknown
mkdir tmp
wasm-bindgen target/wasm32-unknown-unknown/release/parse_wiki_text_sandbox.wasm --no-modules --no-typescript --out-dir tmp
wasm-gc tmp/parse_wiki_text_sandbox_bg.wasm app/app.wasm
rm tmp/parse_wiki_text_sandbox_bg.wasm
mv tmp/parse_wiki_text_sandbox.js app/app.js
rmdir tmp
