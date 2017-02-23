extern crate jsonapi;
extern crate serde_json;

use jsonapi::api::*;

mod helper;
use helper::read_json_file;

#[test]
fn it_works() {
    let resource = Resource {
        _type: format!("test"),
        id: format!("123"),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
    };

    assert_eq!(resource.id, format!("123"));

    let serialized = serde_json::to_string(&resource).unwrap();
    let deserialized: Resource = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.id, resource.id);

    let jsonapidocument = JsonApiDocument {
        data: PrimaryData::None,
        errors: None,
        meta: None,
        included: None,
        links: None,
        jsonapi: None,
    };

    assert_eq!(jsonapidocument.is_valid(), false);

}

#[test]
fn jsonapi_document_can_be_valid() {
    let resource = Resource {
        _type: format!("test"),
        id: format!("123"),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
    };

    let errors = JsonApiErrors::new();

    let jsonapi_document_with_data = JsonApiDocument {
        data: PrimaryData::Single(resource),
        errors: None,
        meta: None,
        included: None,
        links: None,
        jsonapi: None,
    };

    assert_eq!(jsonapi_document_with_data.is_valid(), true);

    let jsonapi_document_with_errors = JsonApiDocument {
        data: PrimaryData::None,
        errors: Some(errors),
        meta: None,
        included: None,
        links: None,
        jsonapi: None,
    };

    assert_eq!(jsonapi_document_with_errors.is_valid(), true);

}

#[test]
fn jsonapi_document_invalid_errors() {

    let resource = Resource {
        _type: format!("test"),
        id: format!("123"),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
    };

    let included_resource = Resource {
        _type: format!("test"),
        id: format!("123"),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
    };

    let errors = JsonApiErrors::new();

    let no_content_document = JsonApiDocument {
        data: PrimaryData::None,
        errors: None,
        meta: None,
        included: None,
        links: None,
        jsonapi: None,
    };

    match no_content_document.validate() {
        None => assert!(false),
        Some(errors) => {
            assert!(errors.contains(&DocumentValidationError::MissingContent));
        }
    }

    let mixed_errors_and_data_document = JsonApiDocument {
        data: PrimaryData::Single(resource),
        errors: Some(errors),
        meta: None,
        included: None,
        links: None,
        jsonapi: None,
    };

    match mixed_errors_and_data_document.validate() {
        None => assert!(false),
        Some(errors) => {
            assert!(errors.contains(&DocumentValidationError::DataWithErrors));
        }
    }

    let included_without_data_document = JsonApiDocument {
        data: PrimaryData::None,
        errors: None,
        meta: None,
        included: Some(vec![included_resource]),
        links: None,
        jsonapi: None,
    };

    match included_without_data_document.validate() {
        None => assert!(false),
        Some(errors) => {
            assert!(errors.contains(&DocumentValidationError::IncludedWithoutData));
        }
    }


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
        r#"{ "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }"#;
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn multiple_resource_from_json_string() {
    let serialized = r#"[
            { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
            { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
            { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }
        ]"#;
    let data: Result<Resources, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn no_data_document_from_json_string() {
    let serialized = r#"{
            "data" : null
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn single_data_document_from_json_string() {
    let serialized = r#"{
            "data" : {
                "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {}
            }
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn multiple_data_document_from_json_string() {
    let serialized = r#"{
            "data" : [
                { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
                { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
                { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }
            ]
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn api_document_from_json_file() {

    let s = ::read_json_file("data/results.json");
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&s);

    match data {
        Ok(res) => {
            match res.data {
                PrimaryData::Multiple(arr) => {
                    assert_eq!(arr.len(), 1);
                }
                PrimaryData::Single(_) => {
                    println!("api_document_from_json_file : Expected one Resource in a vector, \
                              not a direct Resource");
                    assert!(false);
                }
                PrimaryData::None => {
                    println!("api_document_from_json_file : Expected one Resource in a vector");
                    assert!(false);
                }
            }
        }
        Err(err) => {
            println!("api_document_from_json_file : Error: {:?}", err);
            assert!(false);
        }
    }
}

#[test]
fn api_document_collection_from_json_file() {

    let s = ::read_json_file("data/collection.json");
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&s);

    match data {
        Ok(res) => {

            match res.data {
                PrimaryData::Multiple(arr) => {
                    assert_eq!(arr.len(), 1);
                }
                PrimaryData::Single(_) => {
                    println!("api_document_collection_from_json_file : Expected one Resource in \
                              a vector, not a direct Resource");
                    assert!(false);
                }
                PrimaryData::None => {
                    println!("api_document_collection_from_json_file : Expected one Resource in \
                              a vector");
                    assert!(false);
                }
            }

            match res.included {
                Some(arr) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0].id, "9");
                    assert_eq!(arr[1].id, "5");
                    assert_eq!(arr[2].id, "12");
                }
                None => {
                    println!("api_document_collection_from_json_file : Expected three Resources \
                              in 'included' in a vector");
                    assert!(false);
                }
            }

            match res.links {
                Some(links) => {
                    assert_eq!(links.len(), 3);
                }
                None => {
                    println!("api_document_collection_from_json_file : expected links");
                    assert!(false);
                }
            }

        }
        Err(err) => {
            println!("api_document_collection_from_json_file : Error: {:?}", err);
            assert!(false);
        }
    }
}

#[test]
fn can_deserialize_jsonapi_example_resource_001() {
    let s = ::read_json_file("data/resource_001.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_resource_002() {
    let s = ::read_json_file("data/resource_002.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_resource_003() {
    let s = ::read_json_file("data/resource_003.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_compound_document() {
    let s = ::read_json_file("data/compound_document.json");
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_links_001() {
    let s = ::read_json_file("data/links_001.json");
    let data: Result<Links, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_links_002() {
    let s = ::read_json_file("data/links_002.json");
    let data: Result<Links, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_jsonapi_info() {
    let s = ::read_json_file("data/jsonapi_info_001.json");
    let data: Result<JsonApiInfo, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}
