# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Building and Testing
- `cargo build` - Build all workspace members
- `cargo test` - Run all tests across the workspace
- `cargo test -p example` - Run tests for the example crate specifically
- `cargo check` - Fast compilation check for all crates

### Development
- `cargo run --bin example` - Run the example (if it has a binary target)
- `cargo doc --open` - Generate and open documentation

## Architecture

This is a Rust workspace implementing an "enum cast" library that provides type-safe enum operations. The project is organized into three crates:

### Core Architecture
- **enum-cast**: The main library crate that contains the core traits `HasVariant<T>` and `IsSubsetOf<Other>`, and re-exports the derive macro
- **enum-cast-derive**: A procedural macro crate that automatically generates implementations of the core traits for enums
- **example**: Demonstrates usage patterns and serves as integration tests

### Key Concepts
The library enables:
1. **Type-safe variant access**: The `HasVariant<T>` trait provides `make()`, `get()`, and `take()` methods for working with enum variants
2. **Subset relationships**: The `IsSubsetOf<Other>` trait allows safe upcasting (`upcast()`) and downcasting (`downcast()`) between enums where one is a subset of another
3. **Automatic implementation**: The `#[derive(EnumCast)]` macro generates all necessary trait implementations

### Procedural Macro Details
The derive macro in `enum-cast-derive/src/lib.rs:6` generates:
- `HasVariant<T>` implementations for each variant type
- `IsSubsetOf<Other>` implementation with proper bounds checking
- Chain-based downcasting logic that tries each variant type in sequence

### Example Usage Pattern
See `example/src/lib.rs:1` for comprehensive usage examples showing variant access, subset relationships, and downcasting operations.