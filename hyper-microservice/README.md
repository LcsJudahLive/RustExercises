# Rust Hyper Microservice

Implementing example of book "Hand-On Microservices with Rust" until logging setup (pg. 133).

## Pre-Reqs

- `rust` installed with `cargo` and all other binary dependencies

## To Run

- Simple:
```bash
    RUST_LOG=hyper_microservice=debug cargo run
```


- With Cargo Watch:
```bash
    cargo install cargo-watch
    RUST_LOG=hyper_microservice=debug cargo watch -x "run"
```
