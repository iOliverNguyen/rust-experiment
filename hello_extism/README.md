# Experiment with extism

## Experiment

- [x] Create plugin
- [x] Create host and call plugin functions
- [x] Encode with msgpack and bincode
- [ ] Call host functions

## Build & Run

```sh
cd rust-experiment/hello_extism

# guest
cd guest
cargo build --target wasm32-unknown-unknown

# host
cd host
cargo run
```
