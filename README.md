# Mbedtls.js-sys
For people who want to use mbedtls inside Deno or a webbrowser.

Might enable you to:
1. Do DTLS / WebRTC things inside Deno
2. Parse x509 WebRTC certificates within a webbrowser

# How to build:
To build you'll need Rust, and a version of clang that supports wasm32.
```
cargo run --package builder
```
