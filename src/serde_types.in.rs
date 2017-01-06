pub mod jsonapi {
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResourceIdentifier {
        #[serde(rename = "type")]
        pub _type: String,
        pub id: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Resource {
        #[serde(rename = "type")]
        pub _type: String,
        pub id: String,
        pub attributes: ResourceAttributes,
        pub relationships: Relationships,
        pub links: Links,
    }

    pub type Resources = Vec<Resource>;
    pub type Links = HashMap<String, String>;
    pub type Meta = HashMap<String, String>;
    pub type ResourceAttributes = HashMap<String, String>;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Relationship {
    }

    pub type Relationships = Vec<Relationship>;

    pub type Included = Vec<Resource>;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum PrimaryData {
        None,
        Resource,
        Resources,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ErrorSource {
        pointer: Option<String>,
        parameter: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Error {
        id: String,
        links: Links,
        status: String,
        code: String,
        title: String,
        detail: String,
        source: ErrorSource,
        meta: Meta,
    }

    pub type Errors = Vec<Error>;

    // Spec says at least one of data, errors, meta
    // data and errors must not co-exist
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Document {
        data: Option<PrimaryData>,
        errors: Option<Errors>,
        meta: Option<Meta>,
    }
}
