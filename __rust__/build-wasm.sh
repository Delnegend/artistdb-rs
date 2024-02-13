#!/bin/sh

rm -rf composables/wasm
cd __rust__/wasm
rm -rf pkg
wasm-pack build --release
mv pkg ../../composables/wasm
cd ../../

