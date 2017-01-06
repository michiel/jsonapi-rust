extern crate serde;
extern crate serde_json;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let resource = ::Resource {
            _type: format!("test"),
            id: format!("123"),
            attributes: ::ResourceAttributes::new(),
            relationships: ::Relationships::new(),
            links: ::Links::new(),
        };
    }
}
