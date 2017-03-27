#![doc(html_root_url = "https://michiel.github.io/jsonapi-rust/")]

//! This is documentation for the `jsonapi` crate.
//! The crate is meant to be used for serializing, deserializing and validating JSON-API requests and responses.
//!

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate queryst;

#[macro_use]
extern crate log;

pub mod api;
pub mod query;
pub mod model;
