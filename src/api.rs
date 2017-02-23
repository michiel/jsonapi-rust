use serde_json;
use std::collections::HashMap;

pub type JsonApiValue = serde_json::Value;
pub type Resources = Vec<Resource>;
pub type ResourceIdentifiers = Vec<ResourceIdentifier>;
pub type Links = HashMap<String, JsonApiValue>;
pub type Meta = HashMap<String, JsonApiValue>;
pub type ResourceAttributes = HashMap<String, JsonApiValue>;
pub type Relationships = HashMap<String, Relationship>;
pub type Included = Vec<Resource>;
pub type JsonApiErrors = Vec<JsonApiError>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ResourceIdentifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Resource {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub attributes: ResourceAttributes,
    pub relationships: Option<Relationships>,
    pub links: Option<Links>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Relationship {
    pub data: IdentifierData,
    pub links: Option<Links>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PrimaryData {
    None,
    Single(Resource),
    Multiple(Resources),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum IdentifierData {
    None,
    Single(ResourceIdentifier),
    Multiple(ResourceIdentifiers),
}

/// The specification refers to this as a top-level `document`
/// The spec dictates that the document must have least one of `data`, `errors` or `meta`.
/// Of these, `data` and `errors` must not co-exist.
/// The optional field `included` may only be present if the `data` field is present too.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonApiDocument {
    pub data: PrimaryData,
    pub included: Option<Resources>,
    pub links: Option<Links>,
    pub meta: Option<Meta>,
    pub errors: Option<JsonApiErrors>,
    pub jsonapi: Option<JsonApiInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonApiError {
    pub id: String,
    pub links: Links,
    pub status: String,
    pub code: String,
    pub title: String,
    pub detail: String,
    pub source: Option<ErrorSource>,
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonApiInfo {
    pub version: Option<String>,
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

impl Pagination {
    pub fn has_first(&self) -> bool {
        !self.first.is_none()
    }
    pub fn has_prev(&self) -> bool {
        !self.prev.is_none()
    }
    pub fn has_next(&self) -> bool {
        !self.next.is_none()
    }
    pub fn has_last(&self) -> bool {
        !self.last.is_none()
    }
}

impl JsonApiDocument {
    fn has_errors(&self) -> bool {
        !self.errors.is_none()
    }
    fn has_meta(&self) -> bool {
        !self.errors.is_none()
    }
    fn has_included(&self) -> bool {
        !self.included.is_none()
    }
    fn has_data(&self) -> bool {
        match self.data {
            PrimaryData::None => false,
            _ => true,
        }
    }
    pub fn is_valid(&self) -> bool {
        match self.validate() {
            Some(_) => false,
            None => true,
        }
    }

    pub fn validate(&self) -> Option<Vec<DocumentValidationError>> {

        let mut errors = Vec::<DocumentValidationError>::new();

        if self.has_data() && self.has_errors() {
            errors.push(DocumentValidationError::DataWithErrors);
        }

        if self.has_included() && !self.has_data() {
            errors.push(DocumentValidationError::IncludedWithoutData);
        }

        if !(self.has_data() || self.has_meta() || self.has_errors()) {
            errors.push(DocumentValidationError::MissingContent);
        }

        match errors.len() {
            0 => None,
            _ => Some(errors),
        }

    }
}

#[derive(Debug, PartialEq)]
pub enum DocumentValidationError {
    IncludedWithoutData,
    DataWithErrors,
    MissingContent,
}
