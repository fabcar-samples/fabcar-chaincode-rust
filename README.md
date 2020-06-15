# fabcar-rust

A more fab Fabcar in Rust

## Prereqs

To build a Wasm binary you will need to have rust installed, and you will need to add the wasm target.

```
rustup target add wasm32-unknown-unknown
```

## Build

The following command will build a `target/wasm32-unknown-unknown/debug/fabcar_rust.wasm` binary.

```
cargo build --target wasm32-unknown-unknown
```
