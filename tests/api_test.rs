extern crate jsonapi;
extern crate serde_json;
extern crate env_logger;

use jsonapi::api::*;

mod helper;
use crate::helper::read_json_file;

#[test]
fn it_works() {
    let _ = env_logger::try_init();
    let resource = Resource {
        _type: "test".into(),
        id: "123".into(),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
        meta: Some(Meta::new()),
    };

    assert_eq!(resource.id, "123");

    let serialized = serde_json::to_string(&resource).unwrap();
    let deserialized: Resource = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.id, resource.id);

    let jsonapidocument = JsonApiDocument {
        data: Some(PrimaryData::None),
        ..Default::default()
    };

    assert_eq!(jsonapidocument.is_valid(), true);

}

#[test]
fn the_members_data_and_errors_must_not_coexist() {
    use serde_json::Value;

    let jsonapidocument = JsonApiDocument {
        data: Some(PrimaryData::None),
        ..Default::default()
    };

    let string = serde_json::to_string(&jsonapidocument).unwrap();
    let json: Value = serde_json::from_str(&string).unwrap();
    assert_eq!(json.get("data").unwrap().is_null(), true);
    assert_eq!(json.get("errors"), None);

    let errors = JsonApiErrors::new();
    let jsonapi_document_with_errors = JsonApiDocument {
        errors: Some(errors),
        ..Default::default()
    };

    let string = serde_json::to_string(&jsonapi_document_with_errors).unwrap();
    let json: Value = serde_json::from_str(&string).unwrap();
    assert_eq!(json.get("data"), None);
    assert_eq!(json.get("errors").unwrap().is_array(), true);
}

#[test]
fn jsonapi_document_can_be_valid() {
    let _ = env_logger::try_init();
    let resource = Resource {
        _type: "test".into(),
        id: "123".into(),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
        meta: Some(Meta::new()),
    };

    let errors = JsonApiErrors::new();

    let jsonapi_document_with_data = JsonApiDocument {
        data: Some(PrimaryData::Single(Box::new(resource))),
        ..Default::default()
    };

    assert_eq!(jsonapi_document_with_data.is_valid(), true);

    let jsonapi_document_with_errors = JsonApiDocument {
        data: Some(PrimaryData::None),
        errors: Some(errors),
        ..Default::default()
    };

    assert_eq!(jsonapi_document_with_errors.is_valid(), false);
}

#[test]
fn jsonapi_document_invalid_errors() {
    let _ = env_logger::try_init();

    let resource = Resource {
        _type: "test".into(),
        id: "123".into(),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
        meta: Some(Meta::new()),
    };

    let included_resource = Resource {
        _type: "test".into(),
        id: "123".into(),
        attributes: ResourceAttributes::new(),
        relationships: Some(Relationships::new()),
        links: None,
        meta: Some(Meta::new()),
    };

    let errors = JsonApiErrors::new();

    let no_content_document: JsonApiDocument = Default::default();

    match no_content_document.validate() {
        None => assert!(false),
        Some(errors) => {
            assert!(errors.contains(&DocumentValidationError::MissingContent));
        }
    }

    let null_data_content_document = JsonApiDocument {
        data: Some(PrimaryData::None),
        ..Default::default()
    };

    match null_data_content_document.validate() {
        None => assert!(true),
        Some(_) => assert!(false),
    }

    let mixed_errors_and_data_document = JsonApiDocument {
        data: Some(PrimaryData::Single(Box::new(resource))),
        errors: Some(errors),
        ..Default::default()
    };

    match mixed_errors_and_data_document.validate() {
        None => assert!(false),
        Some(errors) => {
            assert!(errors.contains(&DocumentValidationError::DataWithErrors));
        }
    }

    let included_without_data_document = JsonApiDocument {
        included: Some(vec![included_resource]),
        ..Default::default()
    };

    match included_without_data_document.validate() {
        None => assert!(false),
        Some(errors) => {
            assert!(errors.contains(
                &DocumentValidationError::IncludedWithoutData,
            ));
        }
    }
}

#[test]
fn error_from_json_string() {
    let _ = env_logger::try_init();

    let serialized = r#"
        {"id":"1", "links" : {}, "status" : "unknown", "code" : "code1", "title" : "error-title", "detail": "error-detail"}
        "#;
    let error: Result<JsonApiError, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(error.is_ok(), true);
    match error {
        Ok(jsonapierror) => {
            match jsonapierror.id {
                Some(id) => assert_eq!(id, "1"),
                None => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn single_resource_from_json_string() {
    let _ = env_logger::try_init();
    let serialized =
        r#"{ "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }"#;
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn multiple_resource_from_json_string() {
    let _ = env_logger::try_init();
    let serialized = r#"[
            { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
            { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
            { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }
        ]"#;
    let data: Result<Resources, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn no_data_document_from_json_string() {
    let _ = env_logger::try_init();
    let serialized = r#"{
            "data" : null
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn single_data_document_from_json_string() {
    let _ = env_logger::try_init();
    let serialized = r#"{
            "data" : {
                "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {}
            }
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn multiple_data_document_from_json_string() {
    let _ = env_logger::try_init();
    let serialized = r#"{
            "data" : [
                { "id" :"1", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
                { "id" :"2", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} },
                { "id" :"3", "type" : "post", "attributes" : {}, "relationships" : {}, "links" : {} }
            ]
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(data.is_ok(), true);
}

#[test]
fn api_document_from_json_file() {
    let _ = env_logger::try_init();

    let s = crate::read_json_file("data/results.json");
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&s);

    match data {
        Ok(res) => {
            match res.data {
                Some(PrimaryData::Multiple(arr)) => {
                    assert_eq!(arr.len(), 1);
                }
                Some(PrimaryData::Single(_)) => {
                    println!(
                        "api_document_from_json_file : Expected one Resource in a vector, \
                              not a direct Resource"
                    );
                    assert!(false);
                }
                Some(PrimaryData::None) => {
                    println!("api_document_from_json_file : Expected one Resource in a vector");
                    assert!(false);
                }
                None => assert!(false),
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
    let _ = env_logger::try_init();

    let s = crate::read_json_file("data/collection.json");
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&s);

    match data {
        Ok(res) => {

            match res.data {
                Some(PrimaryData::Multiple(arr)) => {
                    assert_eq!(arr.len(), 1);
                }
                Some(PrimaryData::Single(_)) => {
                    println!(
                        "api_document_collection_from_json_file : Expected one Resource in \
                              a vector, not a direct Resource"
                    );
                    assert!(false);
                }
                Some(PrimaryData::None) => {
                    println!(
                        "api_document_collection_from_json_file : Expected one Resource in \
                              a vector"
                    );
                    assert!(false);
                }
                None => assert!(false),
            }

            match res.included {
                Some(arr) => {
                    assert_eq!(arr.len(), 3);
                    assert_eq!(arr[0].id, "9");
                    assert_eq!(arr[1].id, "5");
                    assert_eq!(arr[2].id, "12");
                }
                None => {
                    println!(
                        "api_document_collection_from_json_file : Expected three Resources \
                              in 'included' in a vector"
                    );
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
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/resource_001.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_resource_002() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/resource_002.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_resource_003() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/resource_003.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_resource_004() {
    let _ = env_logger::try_init();
    let s = ::read_json_file("data/resource_004.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_compound_document() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/compound_document.json");
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_links_001() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/links_001.json");
    let data: Result<Links, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_links_002() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/links_002.json");
    let data: Result<Links, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_deserialize_jsonapi_example_jsonapi_info() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/jsonapi_info_001.json");
    let data: Result<JsonApiInfo, serde_json::Error> = serde_json::from_str(&s);
    assert!(data.is_ok());
}

#[test]
fn can_get_attribute() {
    let _ = env_logger::try_init();
    let s = crate::read_json_file("data/resource_all_attributes.json");
    let data: Result<Resource, serde_json::Error> = serde_json::from_str(&s);
    match data {
        Err(_) => assert!(false),
        Ok(res) => {
            match res.get_attribute("likes") {
                None => assert!(false),
                Some(val) => {
                    match val.as_i64() {
                        None => assert!(false),
                        Some(num) => {
                            let x: i64 = 250;
                            assert_eq!(num, x);
                        }
                    }
                }
            }

            match res.get_attribute("title") {
                None => assert!(false),
                Some(val) => {
                    match val.as_str() {
                        None => assert!(false),
                        Some(s) => {
                            assert_eq!(s, "Rails is Omakase");
                        }
                    }
                }
            }

            match res.get_attribute("published") {
                None => assert!(false),
                Some(val) => {
                    match val.as_bool() {
                        None => assert!(false),
                        Some(b) => {
                            assert_eq!(b, true);
                        }
                    }
                }
            }

            match res.get_attribute("tags") {
                None => assert!(false),
                Some(val) => {
                    match val.as_array() {
                        None => assert!(false),
                        Some(arr) => {
                            assert_eq!(arr[0], "rails");
                            assert_eq!(arr[1], "news");
                        }
                    }
                }
            }

        }
    }
}

#[test]
fn can_diff_resource() {
    let _ = env_logger::try_init();
    let s1 = crate::read_json_file("data/resource_post_001.json");
    let s2 = crate::read_json_file("data/resource_post_002.json");

    let data1: Result<Resource, serde_json::Error> = serde_json::from_str(&s1);
    let data2: Result<Resource, serde_json::Error> = serde_json::from_str(&s2);

    match data1 {
        Err(_) => assert!(false),
        Ok(res1) => {
            // So far so good
            match data2 {
                Err(_) => assert!(false),
                Ok(res2) => {
                    match res1.diff(res2) {
                        Err(_) => {
                            assert!(false);
                        }
                        Ok(patchset) => {
                            println!("can_diff_resource: PatchSet is {:?}", patchset);
                            assert_eq!(patchset.patches.len(), 5);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn it_omits_empty_document_and_primary_data_keys() {
    let _ = env_logger::try_init();
    let resource = Resource {
        _type: "test".into(),
        id: "123".into(),
        attributes: ResourceAttributes::new(),
        ..Default::default()
    };
    let doc = JsonApiDocument {
        data: Some(PrimaryData::Single(Box::new(resource))),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_string(&doc).unwrap(),
        r#"{"data":{"type":"test","id":"123","attributes":{}}}"#
    );
}

#[test]
fn it_does_not_omit_an_empty_primary_data() {
    let doc = JsonApiDocument {
        data: Some(PrimaryData::None),
        ..Default::default()
    };

    assert_eq!(serde_json::to_string(&doc).unwrap(), r#"{"data":null}"#);
}

#[test]
fn it_omits_empty_error_keys() {
    let error = JsonApiError {
        id: Some("error_id".to_string()),
        ..Default::default()
    };
    let doc = JsonApiDocument {
        errors: Some(vec![error]),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_string(&doc).unwrap(),
        r#"{"errors":[{"id":"error_id"}]}"#
    );
}

#[test]
fn it_allows_for_optional_attributes() {
    let _ = env_logger::try_init();
    let serialized = r#"{
            "data" : {
                "id" :"1", "type" : "post", "relationships" : {}, "links" : {}
            }
        }"#;
    let data: Result<JsonApiDocument, serde_json::Error> = serde_json::from_str(serialized);
    assert_eq!(data.is_ok(), true);
}
