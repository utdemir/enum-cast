# enum-cast

[![Crates.io](https://img.shields.io/crates/v/enum-cast.svg)](https://crates.io/crates/enum-cast)
[![Documentation](https://docs.rs/enum-cast/badge.svg)](https://docs.rs/enum-cast)
[![Build Status](https://github.com/utdemir/enum-cast/workflows/CI/badge.svg)](https://github.com/utdemir/enum-cast/actions)

A Rust library that provides:

- **Derive macro convenience**: Automatic trait implementations via `#[derive(EnumCast)]`
- **Type-safe casting**: `upcast()` and `downcast()` methods for converting between enums with subset relationships
- **Runtime variant access**: Access enum variant information at runtime using TypeIds

## See example

For usage examples, see [example](enum-cast/examples/example.rs).
