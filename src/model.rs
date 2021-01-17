//! Defines the `JsonApiModel` trait. This is primarily used in conjunction with
//! the [`jsonapi_model!`](../macro.jsonapi_model.html) macro to allow arbitrary
//! structs which implement `Deserialize` to be converted to/from a
//! [`JsonApiDocument`](crate::api::JsonApiDocument) or
//! [`Resource`](crate::api::Resource)
pub use std::collections::HashMap;
pub use crate::api::*;
use crate::errors::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value, Map};

/// A trait for any struct that can be converted from/into a
/// [`Resource`](crate::api::Resource). The only requirement is that your
/// struct has an `id: String` field.
/// You shouldn't be implementing JsonApiModel manually, look at the
/// `jsonapi_model!` macro instead.
pub trait JsonApiModel: Serialize
where
    for<'de> Self: Deserialize<'de>,
{
    #[doc(hidden)]
    fn jsonapi_type(&self) -> String;
    #[doc(hidden)]
    fn jsonapi_id(&self) -> String;
    #[doc(hidden)]
    fn relationship_fields() -> Option<&'static [&'static str]>;
    #[doc(hidden)]
    fn build_relationships(&self) -> Option<Relationships>;
    #[doc(hidden)]
    fn build_included(&self) -> Option<Resources>;

    fn from_jsonapi_resource(resource: &Resource, included: &Option<Resources>)
        -> Result<Self>
    {

        let visited_relationships: Vec<&str> = Vec::new();
        Self::from_serializable(Self::resource_to_attrs(resource, included, &visited_relationships))
    }

    /// Create a single resource object or collection of resource
    /// objects directly from 
    /// [`DocumentData`](../api/struct.DocumentData.html). This method
    /// will parse the document (the `data` and `included` resources) in an
    /// attempt to instantiate the calling struct.
    fn from_jsonapi_document(doc: &DocumentData) -> Result<Self> {
        match doc.data.as_ref() {
            Some(primary_data) => {
                match *primary_data {
                    PrimaryData::None => bail!("Document had no data"),
                    PrimaryData::Single(ref resource) => {
                        Self::from_jsonapi_resource(resource, &doc.included)
                    }
                    PrimaryData::Multiple(ref resources) => {
                        let visited_relationships: Vec<&str> = Vec::new();
                        let all: Vec<ResourceAttributes> = resources
                            .iter()
                            .map(|r| Self::resource_to_attrs(r, &doc.included, &visited_relationships))
                            .collect();
                        Self::from_serializable(all)
                    }
                }
            }
            None => bail!("Document had no data"),
        }
    }

    /// Converts the instance of the struct into a
    /// [`Resource`](../api/struct.Resource.html)
    fn to_jsonapi_resource(&self) -> (Resource, Option<Resources>) {
        if let Value::Object(mut attrs) = to_value(self).unwrap() {
            let _ = attrs.remove("id");
            let resource = Resource {
                _type: self.jsonapi_type(),
                id: self.jsonapi_id(),
                relationships: self.build_relationships(),
                attributes: Self::extract_attributes(&attrs),
                ..Default::default()
            };

            (resource, self.build_included())
        } else {
            panic!(format!("{} is not a Value::Object", self.jsonapi_type()))
        }
    }


    /// Converts the struct into a complete
    /// [`JsonApiDocument`](../api/struct.JsonApiDocument.html)
    fn to_jsonapi_document(&self) -> JsonApiDocument {
        let (resource, included) = self.to_jsonapi_resource();
        JsonApiDocument::Data (
            DocumentData {
                data: Some(PrimaryData::Single(Box::new(resource))),
                included,
                ..Default::default()
            }
        )
    }


    #[doc(hidden)]
    fn build_has_one<M: JsonApiModel>(model: &M) -> Relationship {
        Relationship {
            data: Some(IdentifierData::Single(model.as_resource_identifier())),
            links: None
        }
    }

    #[doc(hidden)]
    fn build_has_many<M: JsonApiModel>(models: &[M]) -> Relationship {
        Relationship {
            data: Some(IdentifierData::Multiple(
                models.iter().map(|m| m.as_resource_identifier()).collect()
            )),
            links: None
        }
    }

    #[doc(hidden)]
    fn as_resource_identifier(&self) -> ResourceIdentifier {
        ResourceIdentifier {
            _type: self.jsonapi_type(),
            id: self.jsonapi_id(),
        }
    }

    /* Attribute corresponding to the model is removed from the Map
     * before calling this, so there's no need to ignore it like we do
     * with the attributes that correspond with relationships.
     * */
    #[doc(hidden)]
    fn extract_attributes(attrs: &Map<String, Value>) -> ResourceAttributes {
        attrs
            .iter()
            .filter(|&(key, _)| {
                if let Some(fields) = Self::relationship_fields() {
                    if fields.contains(&key.as_str()) {
                        return false;
                    }
                }
                true
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    #[doc(hidden)]
    fn to_resources(&self) -> Resources {
        let (me, maybe_others) = self.to_jsonapi_resource();
        let mut flattened = vec![me];
        if let Some(mut others) = maybe_others {
            flattened.append(&mut others);
        }
        flattened
    }

    /// When passed a `ResourceIdentifier` (which contains a `type` and `id`)
    /// this will iterate through the collection provided `haystack` in an
    /// attempt to find and return the `Resource` whose `type` and `id`
    /// attributes match
    #[doc(hidden)]
    fn lookup<'a>(needle: &ResourceIdentifier, haystack: &'a [Resource])
        -> Option<&'a Resource>
    {
        for resource in haystack {
            if resource._type == needle._type && resource.id == needle.id {
                return Some(resource);
            }
        }
        None
    }

    /// Return a [`ResourceAttributes`](../api/struct.ResourceAttributes.html)
    /// object that contains the attributes in this `resource`. This will be
    /// called recursively for each `relationship` on the resource in an attempt
    /// to satisfy the properties for the calling struct.
    ///
    /// The last parameter in this function call is `visited_relationships` which is used as this
    /// function is called recursively. This `Vec` contains the JSON:API `relationships` that were
    /// visited when this function was called last. When operating on the root node of the document
    /// this is simply started with an empty `Vec`.
    ///
    /// Tracking these "visited" relationships is necessary to prevent infinite recursion and stack
    /// overflows. This situation can arise when the "included" resource object includes the parent
    /// resource object - it will simply ping pong back and forth unable to acheive a finite
    /// resolution.
    ///
    /// The JSON:API specification doesn't communicate the direction of a relationship.
    /// Furthermore the current implementation of this crate does not establish an object graph
    /// that could be used to traverse these relationships effectively.
    #[doc(hidden)]
    fn resource_to_attrs(resource: &Resource, included: &Option<Resources>, visited_relationships: &Vec<&str>)
        -> ResourceAttributes
    {
        let mut new_attrs = HashMap::new();
        new_attrs.clone_from(&resource.attributes);
        new_attrs.insert("id".into(), resource.id.clone().into());

        // Copy the contents of `visited_relationships` so that we can mutate within the lexical
        // scope of this function call. This is also important so each edge that we follow (the
        // relationship) is not polluted by data from traversing sibling relationships
        let mut this_visited: Vec<&str> = Vec::new();
        for rel in visited_relationships.iter() {
            this_visited.push(rel);
        }

        if let Some(relations) = resource.relationships.as_ref() {
            if let Some(inc) = included.as_ref() {
                for (name, relation) in relations {
                    // If we have already visited this resource object, exit early and do not
                    // recurse through the relations
                    if this_visited.contains(&name.as_str()) {
                        return new_attrs;
                    }
                    // Track that we have visited this relationship to avoid infinite recursion
                    this_visited.push(name);

                    let value = match relation.data {
                        Some(IdentifierData::None) => Value::Null,
                        Some(IdentifierData::Single(ref identifier)) => {
                            let found = Self::lookup(identifier, inc)
                                .map(|r| Self::resource_to_attrs(r, included, &this_visited) );
                            to_value(found)
                                .expect("Casting Single relation to value")
                        },
                        Some(IdentifierData::Multiple(ref identifiers)) => {
                            let found: Vec<Option<ResourceAttributes>> =
                                identifiers.iter().map(|identifier|{
                                    Self::lookup(identifier, inc).map(|r|{
                                        Self::resource_to_attrs(r, included, &this_visited)
                                    })
                                }).collect();
                            to_value(found)
                                .expect("Casting Multiple relation to value")
                        },
                        None => Value::Null,
                    };
                    new_attrs.insert(name.to_string(), value);
                }
            }
        }
        new_attrs
    }

    #[doc(hidden)]
    fn from_serializable<S: Serialize>(s: S) -> Result<Self> {
        from_value(to_value(s)?).map_err(Error::from)
    }
}

/// Converts a `vec!` of structs into
/// [`Resources`](crate::api::Resources)
///
pub fn vec_to_jsonapi_resources<T: JsonApiModel>(
    objects: Vec<T>,
) -> (Resources, Option<Resources>) {
    let mut included = vec![];
    let resources = objects
        .iter()
        .map(|obj| {
            let (res, mut opt_incl) = obj.to_jsonapi_resource();
            if let Some(ref mut incl) = opt_incl {
                included.append(incl);
            }
            res
        })
        .collect::<Vec<_>>();
    let opt_included = if included.is_empty() {
        None
    } else {
        Some(included)
    };
    (resources, opt_included)
}

/// Converts a `vec!` of structs into a
/// [`JsonApiDocument`](crate::api::JsonApiDocument)
///
/// ```rust
/// #[macro_use] extern crate serde_derive;
/// #[macro_use] extern crate jsonapi;
/// use jsonapi::api::*;
/// use jsonapi::model::*;
///
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// struct Flea {
///     id: String,
///     name: String,
/// }
///
/// jsonapi_model!(Flea; "flea");
///
/// let fleas = vec![
///     Flea {
///         id: "2".into(),
///         name: "rick".into(),
///     },
///     Flea {
///         id: "3".into(),
///         name: "morty".into(),
///     },
/// ];
/// let doc = vec_to_jsonapi_document(fleas);
/// assert!(doc.is_valid());
/// ```
pub fn vec_to_jsonapi_document<T: JsonApiModel>(objects: Vec<T>) -> JsonApiDocument {
    let (resources, included) = vec_to_jsonapi_resources(objects);
    JsonApiDocument::Data (
        DocumentData {
            data: Some(PrimaryData::Multiple(resources)),
            included,
            ..Default::default()
        }
    )
}

impl<M: JsonApiModel> JsonApiModel for Box<M> {
    fn jsonapi_type(&self) -> String {
        self.as_ref().jsonapi_type()
    }

    fn jsonapi_id(&self) -> String {
        self.as_ref().jsonapi_id()
    }

    fn relationship_fields() -> Option<&'static [&'static str]> {
        M::relationship_fields()
    }

    fn build_relationships(&self) -> Option<Relationships> {
        self.as_ref().build_relationships()
    }

    fn build_included(&self) -> Option<Resources> {
        self.as_ref().build_included()
    }
}

/// When applied this macro implements the
/// [`JsonApiModel`](crate::api::JsonApiModel) trait for the provided type
///
#[macro_export]
macro_rules! jsonapi_model {
    ($model:ty; $type:expr) => (
        impl JsonApiModel for $model {
            fn jsonapi_type(&self) -> String { $type.to_string() }
            fn jsonapi_id(&self) -> String { self.id.to_string() }
            fn relationship_fields() -> Option<&'static [&'static str]> { None }
            fn build_relationships(&self) -> Option<Relationships> { None }
            fn build_included(&self) -> Option<Resources> { None }
        }
    );
    ($model:ty; $type:expr;
        has one $( $has_one:ident ),*
    ) => (
        jsonapi_model!($model; $type; has one $( $has_one ),*; has many; has optional);
    );
    ($model:ty; $type:expr;
        has many $( $has_many:ident ),*
    ) => (
        jsonapi_model!($model; $type; has one; has many $( $has_many ),*; has optional);
    );
    ($model:ty; $type:expr;
        has one $( $has_one:ident ),*;
        has many $( $has_many:ident ),*;
        has optional $( $has_opt:ident ),*
    ) => (
        impl JsonApiModel for $model {
            fn jsonapi_type(&self) -> String { $type.to_string() }
            fn jsonapi_id(&self) -> String { self.id.to_string() }

            fn relationship_fields() -> Option<&'static [&'static str]> {
                static FIELDS: &'static [&'static str] = &[
                     $( stringify!($has_one),)*
                     $( stringify!($has_many),)*
                     $( stringify!($has_opt),)*
                ];

                Some(FIELDS)
            }

            fn build_relationships(&self) -> Option<Relationships> {
                let mut relationships = HashMap::new();
                $(
                    relationships.insert(stringify!($has_one).into(),
                        Self::build_has_one(&self.$has_one)
                    );
                )*
                $(
                    relationships.insert(
                        stringify!($has_many).into(),
                        {
                            let values = &self.$has_many.get_models();
                            Self::build_has_many(values)
                        }
                    );
                )*
                $(
                    if let Some(model) = &self.$has_opt {
                        relationships.insert(stringify!($has_opt).into(),
                            Self::build_has_one(model)
                        );
                    }
                )*
                Some(relationships)
            }

            fn build_included(&self) -> Option<Resources> {
                let mut included:Resources = vec![];
                $( included.append(&mut self.$has_one.to_resources()); )*
                $(
                    if let Some(resource) = &self.$has_opt {
                        included.append(&mut resource.to_resources());
                    }
                )*
                $(
                    for model in self.$has_many.get_models() {
                        included.append(&mut model.to_resources());
                    }
                )*
                Some(included)
            }
        }
    );
}
