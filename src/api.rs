use serde_json;
use std::collections::HashMap;

/// Permitted JSON-API values (all JSON Values)
pub type JsonApiValue = serde_json::Value;

/// Vector of `Resource`
pub type Resources = Vec<Resource>;
/// Vector of `ResourceIdentifiers`
pub type ResourceIdentifiers = Vec<ResourceIdentifier>;
pub type Links = HashMap<String, JsonApiValue>;
/// Meta-data object, can contain any data
pub type Meta = HashMap<String, JsonApiValue>;
/// Resource Attributes, can be any JSON value
pub type ResourceAttributes = HashMap<String, JsonApiValue>;
/// Map of relationships with other objects
pub type Relationships = HashMap<String, Relationship>;
/// Side-loaded Resources
pub type Included = Vec<Resource>;
/// Data-related errors
pub type JsonApiErrors = Vec<JsonApiError>;

pub type JsonApiId = String;
pub type JsonApiIds<'a> = Vec<&'a JsonApiId>;

/// Resource Identifier
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResourceIdentifier {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: JsonApiId,
}

/// JSON-API Resource
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resource {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: JsonApiId,
    pub attributes: ResourceAttributes,
    pub relationships: Option<Relationships>,
    pub links: Option<Links>,
}

/// Relationship with another object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    pub data: IdentifierData,
    pub links: Option<Links>,
}

/// Valid data Resource (can be None)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PrimaryData {
    None,
    Single(Resource),
    Multiple(Resources),
}

/// Valid Resource Identifier (can be None)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum IdentifierData {
    None,
    Single(ResourceIdentifier),
    Multiple(ResourceIdentifiers),
}

/// The specification refers to this as a top-level `document`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonApiDocument {
    pub data: Option<PrimaryData>,
    pub included: Option<Resources>,
    pub links: Option<Links>,
    pub meta: Option<Meta>,
    pub errors: Option<JsonApiErrors>,
    pub jsonapi: Option<JsonApiInfo>,
}

/// Error location
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

/// JSON-API Error
/// All fields are optional
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonApiError {
    pub id: Option<String>,
    pub links: Option<Links>,
    pub status: Option<String>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub source: Option<ErrorSource>,
    pub meta: Option<Meta>,
}

/// Optional `JsonApiDocument` payload identifying the JSON-API version the server implements
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JsonApiInfo {
    pub version: Option<String>,
    pub meta: Option<Meta>,
}

/// Pagination links
#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}


#[derive(Debug)]
pub struct Patch {
    pub patch_type: PatchType,
    pub subject: String,
    pub previous: JsonApiValue,
    pub next: JsonApiValue,
}

#[derive(Debug)]
pub struct PatchSet {
    pub resource_type: String,
    pub resource_id: String,
    pub patches: Vec<Patch>,
}

impl PatchSet {
    pub fn new_for(resource: &Resource) -> Self {
        PatchSet {
            resource_type: resource._type.clone(),
            resource_id: resource.id.clone(),
            patches: Vec::<Patch>::new(),
        }
    }

    pub fn push(&mut self, patch: Patch) -> () {
        self.patches.push(patch);
    }
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
    /// The spec dictates that the document must have least one of `data`, `errors` or `meta`.
    /// Of these, `data` and `errors` must not co-exist.
    /// The optional field `included` may only be present if the `data` field is present too.
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
        serde_json::from_str(s)
    }
}

impl Resource {
    /// Instantiate from string
    ///
    /// ```
    /// use jsonapi::api::Resource;
    ///
    /// let serialized = r#"{
    ///   "id":"1",
    ///   "type":"post",
    ///   "attributes":{
    ///     "title": "Rails is Omakase",
    ///     "likes": 250
    ///   },
    ///   "relationships":{},
    ///   "links" :{}
    /// }"#;
    ///
    /// let data = Resource::from_str(&serialized);
    /// assert_eq!(data.is_ok(), true);
    /// ```
    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn get_relationship(&self, name: &str) -> Option<&Relationship> {
        match self.relationships {
            None => None,
            Some(ref relationships) => {
                match relationships.get(name) {
                    None => None,
                    Some(rel) => Some(rel),
                }
            }
        }
    }

    /// Get an attribute `JsonApiValue`
    ///
    /// ```
    /// use jsonapi::api::Resource;
    ///
    /// let serialized = r#"{
    ///   "id":"1",
    ///   "type":"post",
    ///   "attributes":{
    ///     "title": "Rails is Omakase",
    ///     "likes": 250
    ///   },
    ///   "relationships":{},
    ///   "links" :{}
    /// }"#;
    ///
    /// match Resource::from_str(&serialized) {
    ///   Err(_)=> assert!(false),
    ///   Ok(resource)=> {
    ///     match resource.get_attribute("title") {
    ///       None => assert!(false),
    ///       Some(attr) => {
    ///         match attr.as_str() {
    ///           None => assert!(false),
    ///           Some(s) => {
    ///               assert_eq!(s, "Rails is Omakase");
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    pub fn get_attribute(&self, name: &str) -> Option<&JsonApiValue> {
        match self.attributes.get(name) {
            None => None,
            Some(val) => Some(val),
        }
    }

    pub fn get_attribute_as_string(&self, attr: &str) -> Result<String, JsonApiDataError> {
        match self.attributes.get(attr) {
            None => Err(JsonApiDataError::AttributeNotFound),
            Some(json_attr) => {
                match json_attr.as_str() {
                    None => Err(JsonApiDataError::IncompatibleAttributeType),
                    Some(s) => Ok(s.into()),
                }
            }
        }
    }

    pub fn get_attribute_as_i64(&self, attr: &str) -> Result<i64, JsonApiDataError> {
        match self.attributes.get(attr) {
            None => Err(JsonApiDataError::AttributeNotFound),
            Some(json_attr) => {
                match json_attr.as_i64() {
                    None => Err(JsonApiDataError::IncompatibleAttributeType),
                    Some(s) => Ok(s.into()),
                }
            }
        }
    }

    pub fn diff(&self, other: Resource) -> Result<PatchSet, DiffPatchError> {
        if self._type != other._type {
            Err(DiffPatchError::IncompatibleTypes(self._type.clone(), other._type.clone()))
        } else {

            let mut self_keys: Vec<String> =
                self.attributes.iter().map(|(key, _)| key.clone()).collect();

            self_keys.sort();

            let mut other_keys: Vec<String> =
                other.attributes.iter().map(|(key, _)| key.clone()).collect();

            other_keys.sort();

            let matching = self_keys.iter().zip(other_keys.iter()).filter(|&(a, b)| a == b).count();

            if matching != self_keys.len() {
                Err(DiffPatchError::DifferentAttributeKeys)
            } else {
                let mut patchset = PatchSet::new_for(self);

                for (attr, self_value) in self.attributes.iter() {
                    match other.attributes.get(attr) {
                        None => {
                            // XXX This should not happen
                        }
                        Some(other_value) => {
                            if self_value.to_string() != other_value.to_string() {
                                patchset.push(Patch {
                                    patch_type: PatchType::Attribute,
                                    subject: attr.clone(),
                                    previous: self_value.clone(),
                                    next: other_value.clone(),
                                });
                            }
                        }
                    }

                }

                Ok(patchset)
            }
        }
    }

    pub fn patch(&mut self, patchset: PatchSet) -> Result<Resource, DiffPatchError> {
        let mut res = self.clone();
        for patch in &patchset.patches {
            res.attributes.insert(patch.subject.clone(), patch.next.clone());
        }
        Ok(res)
    }
}

impl Relationship {
    pub fn as_id(&self) -> Result<Option<&JsonApiId>, RelationshipAssumptionError> {
        match self.data {
            IdentifierData::None => Ok(None),
            IdentifierData::Multiple(_) => Err(RelationshipAssumptionError::RelationshipIsAList),
            IdentifierData::Single(ref data) => Ok(Some(&data.id)),
        }
    }

    pub fn as_ids(&self) -> Result<Option<JsonApiIds>, RelationshipAssumptionError> {
        match self.data {
            IdentifierData::None => Ok(None),
            IdentifierData::Single(_) => Err(RelationshipAssumptionError::RelationshipIsNotAList),
            IdentifierData::Multiple(ref data) => Ok(Some(data.iter().map(|x| &x.id).collect())),
        }
    }
}

/// Top-level (Document) JSON-API specification violations
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentValidationError {
    IncludedWithoutData,
    DataWithErrors,
    MissingContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonApiDataError {
    AttributeNotFound,
    IncompatibleAttributeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipAssumptionError {
    RelationshipIsAList,
    RelationshipIsNotAList,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiffPatchError {
    IncompatibleTypes(String, String),
    DifferentAttributeKeys,
    NonExistentProperty(String),
    IncorrectPropertyValue(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatchType {
    Relationship,
    Attribute,
}
