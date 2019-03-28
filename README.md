# Tytle

## The Typed-Turtle Programming Language


<img src="http://icons.iconarchive.com/icons/martin-berube/flat-animal/256/turtle-icon.png" height=150 weight=150">

## What's _Tytle_?
Tytle is a variant of the [Logo Programming Language](https://en.wikipedia.org/wiki/Logo_(programming_language)) written in *Rust* and targeted to run in the browser



## Installation
* Clone locally the repository:
```zsh
git clone https://github.com/spacemeshos/tytle
```
* Make sure you have [Rust](https://rustup.rs/) installed
* Make sure you have [npm](https://nodejs.org/) installed
* Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* Make sure you have *python3* installed *it's for spinning a local web-server to host `index.html`)
if you want another web-server, then just replace the `python3 -m http.server` line inside `build.sh` with something else
* Build & Run:

```zsh
cd tytle_browser
./build.sh
```
* Open your browser at http://0.0.0.0:8000/
* Start playing


## Roadmap
* [x] Interpreter
* [ ] Native Compiler (compiling programs directly to `WASM`)
* [ ] Having many turtles (Object-Oriented)
* [ ] String Primitive
* [ ] List Data-Structure
* [ ] HashMap Data-Structure

## License
The `Tytle` project is licensed under Apache License, Version 2.0
