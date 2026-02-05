# Project Tsukuyomi

Simple Rust chat using gRPC with `tonic`. This repository includes:
- `server`: gRPC server exposing `Join` (streaming) and `SendMessage`.
- `client_cli`: command-line client for sending messages and consuming the stream.
- `proto`: shared `chat.proto` definition.

## Requirements
- Rust (stable toolchain)
- Cargo
- Protobuf (required at build time for code generation)

## Structure
- `proto/chat.proto`: gRPC contract
- `server/build.rs`: generates code from the proto via `tonic_prost_build`
- `server/src/main.rs`: gRPC server
- `client_cli/src/main.rs`: CLI client

## How to run

### 1) Server
```bash
cd server
cargo run
```

The server listens on `http://[::1]:50051`.

### 2) Client
In another terminal:
```bash
cd client_cli
cargo run
```

## Basic flow
- The client joins the `Join` stream.
- The client sends messages via `SendMessage`.
- Messages received from the stream are printed to the console.

## Notes
- `tonic` generates files under `target/.../out`, included via `include_proto!`.
- The current server is a functional skeleton and can be extended to broadcast to all users.

