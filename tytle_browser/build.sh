#!/bin/sh

npm install codemirror
wasm-pack build --target web
python3 -m http.server
