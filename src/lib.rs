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
//! The crate is meant to be used for serializing, deserializing and validating
//! [JSON:API] requests and responses.
//!
//! [JSON:API]: https://jsonapi.org/
//! [serde]: https://serde.rs
//! [JsonApiDocument]: api/struct.JsonApiDocument.html
//! [Resource]: api/struct.Resource.html
//! [jsonapi_model]: macro.jsonapi_model.html
//!
//! ## Examples
//!
//! ### Basic Usage with Macro
//!
//! Using the [`jsonapi_model!`][jsonapi_model] macro a struct can be converted
//! into a [`JsonApiDocument`][JsonApiDocument] or [`Resource`][Resource]. It is
//! required that the struct have an `id` property whose type is `String`. The
//! second argument in the [`jsonapi_model!`][jsonapi_model] marco defines the
//! `type` member as required by the [JSON:API] specification
//!
//! ```rust
//! #[macro_use] extern crate serde_derive;
//! #[macro_use] extern crate jsonapi;
//! use jsonapi::api::*;
//! use jsonapi::model::*;
//!
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct Flea {
//!     id: String,
//!     name: String,
//! };
//!
//! jsonapi_model!(Flea; "flea");
//!
//! let example_flea = Flea {
//!     id: "123".into(),
//!     name: "Mr.Flea".into(),
//! };
//!
//! // Convert into a `JsonApiDocument`
//! let doc = example_flea.to_jsonapi_document();
//! assert!(doc.is_valid());
//!
//! // Convert into a `Resource`
//! let resource = example_flea.to_jsonapi_resource();
//! ```
//!
//! ### Deserializing a JSON:API Document
//!
//! Deserialize a JSON:API document using [serde] by explicitly declaring the
//! variable type in `Result`
//!
//! ```ignore
//! let serialized = r#"
//! {
//!   "data": [{
//!     "type": "articles",
//!     "id": "1",
//!     "attributes": {
//!       "title": "JSON:API paints my bikeshed!",
//!       "body": "The shortest article. Ever."
//!     },
//!     "relationships": {
//!       "author": {
//!         "data": {"id": "42", "type": "people"}
//!       }
//!     }
//!   }],
//!   "included": [
//!     {
//!       "type": "people",
//!       "id": "42",
//!       "attributes": {
//!         "name": "John"
//!       }
//!     }
//!   ]
//! }"#;
//! let data: Result<Resource, serde_json::Error> = serde_json::from_str(&serialized);
//! assert_eq!(data.is_ok(), true);
//! ```
//!
//! Or parse the `String` directly using the
//! [Resource::from_str](api/struct.Resource.html) trait implementation
//!
//! ```ignore
//! let data = Resource::from_str(&serialized);
//! assert_eq!(data.is_ok(), true);
//! ```
//!
//! [`JsonApiDocument`][JsonApiDocument] implements `PartialEq` which allows two
//! documents to be compared for equality. If two documents possess the **same
//! contents** the ordering of the attributes and fields within the JSON:API
//! document are irrelevant and their equality will be `true`.
//!
//! ## Testing
//!
//! Run the tests:
//!
//! ```text
//! cargo test
//! ```
//!
//! Run tests with more verbose output:
//!
//! ```text
//! RUST_BACKTRACE=1 cargo test -- --nocapture
//! ```
//!
//! Run tests whenever files are modified using `cargo watch`:
//!
//! ```text
//! RUST_BACKTRACE=1 cargo watch "test -- --nocapture"
//! ```
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
pub mod array;
pub mod query;
pub mod model;
pub mod errors;
