# Slint Studio

A small project dashboard written once in Slint and Rust, with a native Windows desktop build and a WebAssembly build for the browser.

## Windows desktop app

Install the [Rust toolchain](https://rustup.rs/) and the Visual Studio C++ Build Tools, then run:

```powershell
cargo run --release
```

The executable is created at `target\release\slint-dashboard.exe`.

## Web app

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and the WebAssembly target, then from this directory run:

```powershell
rustup target add wasm32-unknown-unknown
wasm-pack build --release --target web
python -m http.server 8000
```

Open <http://localhost:8000> in a browser. The generated `pkg` directory is intentionally ignored by Git.

## UI

The shared interface in `ui/app.slint` contains reusable metric cards, navigation, activity entries, a checkbox, and an interactive project button. Both the desktop binary and the browser build use the same Rust entry point and UI.
