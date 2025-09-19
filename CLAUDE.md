# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Waynest is a foundational library implementing the low-level Wayland wire protocol in Rust. It's built on Tokio's async ecosystem and serves as a building block for higher-level Wayland libraries like Waynova (client) and Verdi (compositor).

## Architecture

The codebase follows a workspace structure with three main components:

- **Main crate (`waynest`)**: Core wire protocol implementation with `Socket` struct implementing `Sink` and `Stream` traits
- **Code generation (`gen/`)**: Contains `waynest-gen` tool for generating protocol bindings from XML interface descriptions
- **Macros (`macros/`)**: Procedural macros including `Dispatcher` trait derivation

### Key Modules

- **Wire protocol**: Core message/payload handling, socket communication (`src/socket.rs`, `src/message.rs`, `src/payload.rs`)
- **Server module**: Generated traits from XML definitions for implementing Wayland protocols (`src/server/`)
- **Client module**: Client-side protocol implementation (currently minimal, `src/client/`)

## Development Commands

### Building and Testing
```bash
# Run tests (CI uses nextest)
cargo nextest run --all-features
# Fallback if nextest not available
cargo test --all-features

# Check code
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features
cargo fmt --all -- --check
```

### Protocol Generation
```bash
# Generate protocol bindings from XML files
./generate-protocols.sh

# Run the generator manually
cargo run -p waynest-gen --features bin
```

### Features

The project uses feature flags for different protocol sets:
- `stable`, `staging`, `unstable`: Standard Wayland protocols
- `wlr`, `plasma`, `weston`, `cosmic`, `frog`, `ivi`, `hyprland`, `mesa`, `treeland`: Vendor-specific protocols

## Protocol Structure

Protocol XML files are organized in `protocols/` directory with submodules for different protocol sources. The `generate-protocols.sh` script processes these into `gen/src/protocols.json` which drives code generation.

## Current Status

The codebase is undergoing refactoring (branch: `splitting`) to move wire protocol components from `src/wire/` to the root `src/` level. Generated protocol code is produced by the `waynest-gen` tool from XML interface descriptions.