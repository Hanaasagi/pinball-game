# Pinball-Game
Online deme see [GitHub Page](https://hanaasagi.github.io/pinball-game/release/)


### Building (using Rust's native WebAssembly backend)

1. Install newest nightly Rust, or you can use docker:
```Bash
$ curl https://sh.rustup.rs -sSf | sh
```
2. Install WebAssembly target:
```Bash
$ rustup target add wasm32-unknown-unknown
```
3. Install [cargo-web](https://github.com/koute/cargo-web):
```Bash
$ cargo install -f cargo-web
```
4. Build it:
```Bash
$ cargo web start --target-webasm --release
````
5. Visit `http://localhost:8000` with your browser which support WebAssembly.
