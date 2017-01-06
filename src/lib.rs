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
}
