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

///
/// JSON-API Resource
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
    pub data: Option<PrimaryData>,
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

/// JSON-API Error
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

/// Optional JsonApiDocument payload identifying the JSON-API version the server implements
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

/// Top-level JSON-API Document
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
        !self.data.is_none()
    }
    /// This function returns `false` if the `JsonApiDocument` contains any violations of the
    /// specification. See `DocumentValidationError`
    ///
    /// ```
    /// use jsonapi::api::{JsonApiDocument, PrimaryData, JsonApiErrors};
    /// let doc = JsonApiDocument {
    ///     data: Some(PrimaryData::None),
    ///     errors: Some(JsonApiErrors::new()),
    ///     meta: None,
    ///     included: None,
    ///     links: None,
    ///     jsonapi: None,
    /// };
    ///
    /// assert_eq!(doc.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        match self.validate() {
            Some(_) => false,
            None => true,
        }
    }

    /// This function returns a `Vec` with identified specification violations enumerated in
    /// `DocumentValidationError`
    ///
    /// ```
    /// use jsonapi::api::{JsonApiDocument, PrimaryData, JsonApiErrors, DocumentValidationError};
    ///
    /// let doc = JsonApiDocument {
    ///     data: Some(PrimaryData::None),
    ///     errors: Some(JsonApiErrors::new()),
    ///     meta: None,
    ///     included: None,
    ///     links: None,
    ///     jsonapi: None,
    /// };
    ///
    /// match doc.validate() {
    ///   Some(errors) => {
    ///     assert!(
    ///       errors.contains(
    ///         &DocumentValidationError::DataWithErrors
    ///       )
    ///     )
    ///   }
    ///   None => assert!(false)
    /// }
    /// ```
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

    /// Instantiate from string
    ///
    /// ```
    /// use jsonapi::api::JsonApiDocument;
    ///
    /// let serialized = r#"{
    ///   "data" : [
    ///     { "id":"1", "type":"post", "attributes":{}, "relationships":{}, "links" :{} },
    ///     { "id":"2", "type":"post", "attributes":{}, "relationships":{}, "links" :{} },
    ///     { "id":"3", "type":"post", "attributes":{}, "relationships":{}, "links" :{} }
    ///   ]
    /// }"#;
    /// let doc = JsonApiDocument::from_str(&serialized);
    /// assert_eq!(doc.is_ok(), true);
    /// ```
    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        let data: Result<Self, serde_json::Error> = serde_json::from_str(&s);
        data
    }
}

/// Top-level (Document) JSON-API specification violations
#[derive(Debug, PartialEq)]
pub enum DocumentValidationError {
    IncludedWithoutData,
    DataWithErrors,
    MissingContent,
}
