# jsonapi-rust

[![Build Status](https://travis-ci.org/michiel/jsonapi-rust.svg?branch=master)](https://travis-ci.org/michiel/jsonapi-rust)

This is an implementation of the JSON-API v1 specification at [jsonapi.org](http://jsonapi.org/).

[Documentation](https://michiel.github.io/jsonapi-rust/)

## Use

Add this crate to your _Cargo.toml_ file,

    [dependencies]
    jsonapi = "0.1.0"

## Test

The command `cargo test` will run all tests. For more verbose output or output with _cargo watch_,

    RUST_BACKTRACE=1 cargo test -- --nocapture
    RUST_BACKTRACE=1 cargo watch "test -- --nocapture"
