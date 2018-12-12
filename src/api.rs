use serde_json;
use std::collections::HashMap;
use errors::*;
use std::str::FromStr;
use std;

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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Resource {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: JsonApiId,
    #[serde(default)]
    pub attributes: ResourceAttributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Relationships>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

/// Relationship with another object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IdentifierData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Valid data Resource (can be None)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PrimaryData {
    None,
    Single(Box<Resource>),
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct JsonApiDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PrimaryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Resources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<JsonApiErrors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<JsonApiInfo>,
}

/// Error location
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ErrorSource {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

/// JSON-API Error
/// All fields are optional
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct JsonApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Top-level JSON-API Document
impl JsonApiDocument {
    fn has_errors(&self) -> bool {
        self.errors.is_some()
    }
    fn has_meta(&self) -> bool {
        self.errors.is_some()
    }
    fn has_included(&self) -> bool {
        self.included.is_some()
    }
    fn has_data(&self) -> bool {
        self.data.is_some()
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
    ///     ..Default::default()
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
    ///     ..Default::default()
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
}

impl FromStr for JsonApiDocument {
    type Err = Error;

    /// Instantiate from string
    ///
    /// ```
    /// use jsonapi::api::JsonApiDocument;
    /// use std::str::FromStr;
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
    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).chain_err(|| "Error parsing Document")
    }
}

impl Resource {
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
    /// use std::str::FromStr;
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

    pub fn diff(&self, other: Resource) -> std::result::Result<PatchSet, DiffPatchError> {
        if self._type != other._type {
            Err(DiffPatchError::IncompatibleTypes(
                self._type.clone(),
                other._type.clone(),
            ))
        } else {

            let mut self_keys: Vec<String> =
                self.attributes.iter().map(|(key, _)| key.clone()).collect();

            self_keys.sort();

            let mut other_keys: Vec<String> = other
                .attributes
                .iter()
                .map(|(key, _)| key.clone())
                .collect();

            other_keys.sort();

            let matching = self_keys
                .iter()
                .zip(other_keys.iter())
                .filter(|&(a, b)| a == b)
                .count();

            if matching != self_keys.len() {
                Err(DiffPatchError::DifferentAttributeKeys)
            } else {
                let mut patchset = PatchSet::new_for(self);

                for (attr, self_value) in &self.attributes {
                    match other.attributes.get(attr) {
                        None => {
                            error!(
                                "Resource::diff unable to find attribute {:?} in {:?}",
                                attr,
                                other
                            );
                        }
                        Some(other_value) => {
                            if self_value != other_value {
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

    pub fn patch(&mut self, patchset: PatchSet) -> Result<Resource> {
        let mut res = self.clone();
        for patch in &patchset.patches {
            res.attributes.insert(
                patch.subject.clone(),
                patch.next.clone(),
            );
        }
        Ok(res)
    }
}

impl FromStr for Resource {
    type Err = Error;

    /// Instantiate from string
    ///
    /// ```
    /// use jsonapi::api::Resource;
    /// use std::str::FromStr;
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
    fn from_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).chain_err(|| "Error parsing resource")
    }
}


impl Relationship {
    pub fn as_id(&self) -> std::result::Result<Option<&JsonApiId>, RelationshipAssumptionError> {
        match self.data {
            Some(IdentifierData::None) => Ok(None),
            Some(IdentifierData::Multiple(_)) => Err(RelationshipAssumptionError::RelationshipIsAList),
            Some(IdentifierData::Single(ref data)) => Ok(Some(&data.id)),
            None => Ok(None),
        }
    }

    pub fn as_ids(&self) -> std::result::Result<Option<JsonApiIds>, RelationshipAssumptionError> {
        match self.data {
            Some(IdentifierData::None) => Ok(None),
            Some(IdentifierData::Single(_)) => Err(RelationshipAssumptionError::RelationshipIsNotAList),
            Some(IdentifierData::Multiple(ref data)) => Ok(Some(data.iter().map(|x| &x.id).collect())),
            None => Ok(None),
        }
    }
}

/// Top-level (Document) JSON-API specification violations
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DocumentValidationError {
    IncludedWithoutData,
    DataWithErrors,
    MissingContent,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum JsonApiDataError {
    AttributeNotFound,
    IncompatibleAttributeType,
}

#[derive(Debug, Clone, PartialEq, Copy)]
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PatchType {
    Relationship,
    Attribute,
}
