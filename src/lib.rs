#![deny(missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications
       )]

#![doc(html_root_url = "https://docs.rs/jsonapi/")]

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

#[macro_use]
extern crate error_chain;

pub mod api;
pub mod query;
pub mod model;
pub mod errors;
