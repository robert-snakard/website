#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/binary.wasm .

git add binary.wasm
git add *
git commit -m "First Wasm!"
git push
