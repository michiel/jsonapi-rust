extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_json;

pub mod api;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_json_file(filename: &str) -> String {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {} // print!("{} contains:\n{}", display, s),
    };

    s
}

#[cfg(test)]
mod tests {
    use super::api::*;

    use serde_json;

    #[test]
    fn it_works() {
        let resource = Resource {
            _type: format!("test"),
            id: format!("123"),
            attributes: ResourceAttributes::new(),
            relationships: Relationships::new(),
            links: Links::new(),
        };

        assert_eq!(resource.id, format!("123"));

        let serialized = serde_json::to_string(&resource).unwrap();
        let deserialized: Resource = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id, resource.id);

        let document = Document {
            data: None,
            errors: None,
            meta: None,
        };

        assert_eq!(document.has_data(), false);
        assert_eq!(document.has_errors(), false);
        assert_eq!(document.has_meta(), false);

        assert_eq!(document.is_valid(), false);

    }

    #[test]
    fn document_can_be_valid() {
        let resource = Resource {
            _type: format!("test"),
            id: format!("123"),
            attributes: ResourceAttributes::new(),
            relationships: Relationships::new(),
            links: Links::new(),
        };

        let errors = JsonApiErrors::new();

        let invalid_document = Document {
            data: None,
            errors: None,
            meta: None,
        };

        assert_eq!(invalid_document.is_valid(), false);

        let document_with_data = Document {
            data: Some(PrimaryData::Single(resource)),
            errors: None,
            meta: None,
        };

        assert_eq!(document_with_data.is_valid(), true);

        let document_with_errors = Document {
            data: None,
            errors: Some(errors),
            meta: None,
        };

        assert_eq!(document_with_errors.is_valid(), true);

    }

    #[test]
    fn error_from_json_string() {

        let serialized = r#"
        {"id":"1", "links" : {}, "status" : "unknown", "code" : "code1", "title" : "error-title", "detail": "error-detail"}
        "#;
        let error: Result<JsonApiError, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(error.is_ok(), true);
        assert_eq!(error.unwrap().id, "1");
    }

    #[test]
    fn single_resource_from_json_string() {
        let serialized =
            r#"{ "id" :"1", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} }"#;
        let data: Result<Resource, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(data.is_ok(), true);
    }

    #[test]
    fn multiple_resource_from_json_string() {
        let serialized = r#"[
            { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} },
            { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} },
            { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} }
        ]"#;
        let data: Result<Resources, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(data.is_ok(), true);
    }

    #[test]
    fn no_data_response_from_json_string() {
        let serialized = r#"{
            "data" : null
        }"#;
        let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(data.is_ok(), true);
    }

    #[test]
    fn single_data_response_from_json_string() {
        let serialized = r#"{
            "data" : {
                "id" :"1", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {}
            }
        }"#;
        let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(data.is_ok(), true);
    }

    #[test]
    fn multiple_data_response_from_json_string() {
        let serialized = r#"{
            "data" : [
                { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} },
                { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} },
                { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : [], "links" : {} }
            ]
        }"#;
        let data: Result<JsonApiResponse, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(data.is_ok(), true);
    }

}
