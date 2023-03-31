# Rust chat server

[![Build Status](https://app.travis-ci.com/ChamathKB/rust-chat-server.svg?token=DqNWbcbaY78hF4ZsimVD&branch=master)](https://app.travis-ci.com/ChamathKB/rust-chat-server)

Chat backend server build from Rust 🦀️.

## Build
```
cargo build
```
## Generate documentation
```
cargo rustdoc -- --no-defaults
```
## Run
run the server on port 9090
```
cargo run
```
## Send messages from your host
open another terminal
```
nc localhost 9090
```
