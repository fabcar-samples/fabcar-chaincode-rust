# fabcar-contract-rust

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

## Deploy

The [fabric-contract-api-rust](https://hyperledgendary.github.io/fabric-contract-api-rust/) documentation includes deployment guides for the [test network](https://hyperledger-fabric.readthedocs.io/en/release-2.2/test_network.html) and [Microfab](https://github.com/ibm-blockchain/microfab).
