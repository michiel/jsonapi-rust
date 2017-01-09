extern crate serde;
extern crate serde_json;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

#[cfg(test)]
mod tests {
    use super::*;

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

        let serialized = serde_json::to_string(&resource).unwrap();
        println!("serialized = {}", serialized);
        let deserialized: Resource = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);

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

        let errors = Errors::new();

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
    fn error_from_json() {

        let serialized = r#"{"id":"1", "links" : {}, "status" : "unknown", "code" : "code1", "title" : "error-title", "detail": "error-detail"}"#;
        let error: Result<Error, serde_json::Error> = serde_json::from_str(&serialized);
        assert_eq!(error.is_ok(), true);
        assert_eq!(error.unwrap().id, "1");
    }
}
