# jsonapi-rust

[![Build Status](https://travis-ci.org/michiel/jsonapi-rust.svg?branch=master)](https://travis-ci.org/michiel/jsonapi-rust)
[![Crates.io Status](http://meritbadge.herokuapp.com/jsonapi)](https://crates.io/crates/jsonapi)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/michiel/jsonapi-rust/master/LICENSE)
[![Documentation](https://docs.rs/jsonapi/badge.svg)](https://docs.rs/jsonapi)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fmichiel%2Fjsonapi-rust.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fmichiel%2Fjsonapi-rust?ref=badge_shield)

This is an implementation of the JSON-API v1 specification at [jsonapi.org](http://jsonapi.org/).

  * [API Documentation at docs.rs](https://docs.rs/jsonapi)
  * [CHANGELOG](https://github.com/michiel/jsonapi-rust/blob/master/CHANGELOG.md)

## Use

Add this crate to your _Cargo.toml_ file,

    [dependencies]
    jsonapi = "*"

Or use the master branch directly from github,

    [dependencies]
    jsonapi = { git = "https://github.com/michiel/jsonapi-rust", branch = "master" }

Examples of most serialization and deserialization cases can be found in the [_tests/_](https://github.com/michiel/jsonapi-rust/tree/master/tests) directory or the [documentation](https://docs.rs/jsonapi).

## Development

### Testing

The command `cargo test` will run all tests. For more verbose output or output with _cargo watch_,

    RUST_BACKTRACE=1 cargo test -- --nocapture
    RUST_BACKTRACE=1 cargo watch "test -- --nocapture"

## Contributing

Contributions are welcome. Please add tests and write commit messages using 
using [conventional](https://github.com/conventional-changelog/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) format. The Changelog is updated using the [clog](https://github.com/clog-tool/clog-cli) tool. The configuration is found in `.clog.toml`.

The current configuration works for commit messages prefixed with `feat:`, `bug:`, `test:`, `doc:` and `refactor:`.




## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fmichiel%2Fjsonapi-rust.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fmichiel%2Fjsonapi-rust?ref=badge_large)
