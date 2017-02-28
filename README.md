# jsonapi-rust

[![Build Status](https://travis-ci.org/michiel/jsonapi-rust.svg?branch=master)](https://travis-ci.org/michiel/jsonapi-rust)
[![Crates.io Status](http://meritbadge.herokuapp.com/jsonapi)](https://crates.io/crates/jsonapi)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/michiel/jsonapi-rust/master/LICENSE)
[![Documentation](https://docs.rs/jsonapi/badge.svg)](https://docs.rs/jsonapi)

This is an implementation of the JSON-API v1 specification at [jsonapi.org](http://jsonapi.org/).

[Documentation](https://docs.rs/jsonapi)

## Use

Add this crate to your _Cargo.toml_ file,

    [dependencies]
    jsonapi = "0.3.0"

Examples of most serialization and deserialization cases can be found in the [_tests/_](https://github.com/michiel/jsonapi-rust/tree/master/tests) directory or the [documentation](https://docs.rs/jsonapi).

## Development

The command `cargo test` will run all tests. For more verbose output or output with _cargo watch_,

    RUST_BACKTRACE=1 cargo test -- --nocapture
    RUST_BACKTRACE=1 cargo watch "test -- --nocapture"
