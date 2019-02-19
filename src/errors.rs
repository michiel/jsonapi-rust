error_chain!{
    foreign_links {
        SerdeJson(serde_json::Error);
    }
    errors {
        ResourceToModelError(t: String) {
            description("Error converting Resource to Model")
            display("Error converting Resource to Model: '{}'", t)
        }
    }
}
