error_chain!{
    errors {
        ResourceToModelError(t: String) {
            description("Error converting Resource to Model")
            display("Error converting Resource to Model: '{}'", t)
        }
    }
}
