#!/bin/sh

cargo +nightly build --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/tytle_browser.wasm --out-dir .

npm install
npm run serve
