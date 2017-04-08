# jsonapi-rust

[![Build Status](https://travis-ci.org/michiel/jsonapi-rust.svg?branch=master)](https://travis-ci.org/michiel/jsonapi-rust)
[![codecov](https://codecov.io/gh/michiel/jsonapi-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/michiel/jsonapi-rust)
[![Crates.io Status](http://meritbadge.herokuapp.com/jsonapi)](https://crates.io/crates/jsonapi)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/michiel/jsonapi-rust/master/LICENSE)
[![Documentation](https://docs.rs/jsonapi/badge.svg)](https://docs.rs/jsonapi)

This is an implementation of the JSON-API v1 specification at [jsonapi.org](http://jsonapi.org/).

[Documentation](https://docs.rs/jsonapi)

## Use

Add this crate to your _Cargo.toml_ file,

    [dependencies]
    jsonapi = "*"

Using git

    [dependencies]
    jsonapi = { git = "https://github.com/michiel/jsonapi-rust", branch = "master" }

Examples of most serialization and deserialization cases can be found in the [_tests/_](https://github.com/michiel/jsonapi-rust/tree/master/tests) directory or the [documentation](https://docs.rs/jsonapi).

## Development

### Testing

The command `cargo test` will run all tests. For more verbose output or output with _cargo watch_,

    RUST_BACKTRACE=1 cargo test -- --nocapture
    RUST_BACKTRACE=1 cargo watch "test -- --nocapture"

### Clippy

To run clippy, find a last known working version of nightly that runs with clippy,

    rustup install nightly-2017-03-01
    rustup run nightly-2017-03-01 cargo install clippy
    rustup run nightly-2017-03-01 cargo clippy

### Dependency scanning

Check for outdated packages

    cargo outdated

Check packages for known vulnerabilities

    cargo audit ; echo $?

### Changes and changelog generation

Commit messages are written using [conventional](https://github.com/conventional-changelog/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) format. The Changelog is updated using the [clog](https://github.com/clog-tool/clog-cli) tool. The configuration is found in `.clog.toml`.


