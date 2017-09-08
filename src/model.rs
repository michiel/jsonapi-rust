pub use std::collections::HashMap;
pub use api::*;
use errors::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value, Map};

/// A trait for any struct that can be converted from/into a Resource.
/// The only requirement is that your struct has an 'id: String' field.
/// You shouldn't be implementing JsonApiModel manually, look at the
/// `jsonapi_model!` macro instead.
pub trait JsonApiModel: Serialize
    where for<'de> Self: Deserialize<'de>
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
        Self::from_serializable(Self::resource_to_attrs(resource, included))
    }

    fn from_jsonapi_document(doc: &JsonApiDocument) -> Result<Self> {
        match doc.data.as_ref() {
            Some(primary_data) => {
                match *primary_data {
                    PrimaryData::None => bail!("Document had no data"),
                    PrimaryData::Single(ref resource) =>
                        Self::from_jsonapi_resource(resource, &doc.included),
                    PrimaryData::Multiple(ref resources) => {
                        let all: Vec<ResourceAttributes> = resources
                            .iter()
                            .map(|r| Self::resource_to_attrs(r, &doc.included))
                            .collect();
                        Self::from_serializable(all)
                    }
                }
            },
            None => bail!("Document had no data")
        }
    }

    fn to_jsonapi_resource(&self) -> (Resource, Option<Resources>) {
        if let Value::Object(mut attrs) = to_value(self).unwrap(){
            let id_value = attrs
                .remove("id")
                .unwrap_or_else(|| Value::String("".into()));

            let resource = Resource{
                _type: self.jsonapi_type(),
                id: from_value(id_value).unwrap_or_else(|_| "".into()),
                relationships: self.build_relationships(),
                attributes: Self::extract_attributes(&attrs),
                ..Default::default()
            };

            (resource, self.build_included())
        }else{
            panic!(format!("{} is not a Value::Object", self.jsonapi_type()))
        }
    }

    
    fn to_jsonapi_document(&self) -> JsonApiDocument {
        let (resource, included) = self.to_jsonapi_resource();
        JsonApiDocument {
            data: Some(PrimaryData::Single(Box::new(resource))),
            included: included,
            ..Default::default()
        }
    }

    
    #[doc(hidden)]
    fn build_has_one<M: JsonApiModel>(model: &M) -> Relationship {
        Relationship{
            data: IdentifierData::Single(model.as_resource_identifier()),
            links: None
        }
    }
    
    #[doc(hidden)]
    fn build_has_many<M: JsonApiModel>(models: &[M]) -> Relationship {
        Relationship{
            data: IdentifierData::Multiple(
                models.iter().map(|m| m.as_resource_identifier()).collect()
            ),
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
        attrs.iter().filter(|&(key, _)|{
            if let Some(fields) = Self::relationship_fields(){
                if fields.contains(&key.as_str()) {
                    return false;
                }
            }
            true
        }).map(|(k,v)|{ (k.clone(), v.clone()) }).collect()
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

    #[doc(hidden)]
    fn lookup<'a>(needle: &ResourceIdentifier, haystack: &'a [Resource])
        -> Option<&'a Resource> 
    {
        for resource in haystack {
            if resource._type == needle._type && resource.id == needle.id {
                return Some(resource)
            }
        }
        None
    }

    #[doc(hidden)]
    fn resource_to_attrs(resource: &Resource, included: &Option<Resources>)
        -> ResourceAttributes 
    {
        let mut new_attrs = HashMap::new();
        new_attrs.clone_from(&resource.attributes);
        new_attrs.insert("id".into(), resource.id.clone().into());

        if let Some(relations) = resource.relationships.as_ref() {
            if let Some(inc) = included.as_ref() {
                for (name, relation) in relations {
                    let value = match relation.data {
                        IdentifierData::None => Value::Null,
                        IdentifierData::Single(ref identifier) => {
                            let found = Self::lookup(identifier, inc)
                                .map(|r| Self::resource_to_attrs(r, included) );
                            to_value(found)
                                .expect("Casting Single relation to value")
                        },
                        IdentifierData::Multiple(ref identifiers) => {
                            let found: Vec<Option<ResourceAttributes>> =
                                identifiers.iter().map(|id|{
                                    Self::lookup(id, inc).map(|r|{
                                        Self::resource_to_attrs(r, included)
                                    })
                                }).collect();
                            to_value(found)
                                .expect("Casting Multiple relation to value")
                        },
                    };
                    new_attrs.insert(name.to_string(), value);
                }
            }
        }

        new_attrs
    }

    #[doc(hidden)]
    fn from_serializable<S: Serialize>(s: S) -> Result<Self> {
        from_value(to_value(s).unwrap())
            .chain_err(|| "Error casting via serde_json")
    }
}

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
        jsonapi_model!($model; $type; has one $( $has_one ),*; has many);
    );
    ($model:ty; $type:expr;
        has many $( $has_many:ident ),*
    ) => (
        jsonapi_model!($model; $type; has one; has many $( $has_many ),*);
    );
    ($model:ty; $type:expr;
        has one $( $has_one:ident ),*;
        has many $( $has_many:ident ),*
    ) => (
        impl JsonApiModel for $model {
            fn jsonapi_type(&self) -> String { $type.to_string() }
            fn jsonapi_id(&self) -> String { self.id.to_string() }

            fn relationship_fields() -> Option<&'static [&'static str]> {
                static FIELDS: &'static [&'static str] = &[
                     $( stringify!($has_one),)*
                     $( stringify!($has_many),)*
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
                    relationships.insert(stringify!($has_many).into(),
                        Self::build_has_many(&self.$has_many)
                    );
                )*
                Some(relationships)
            }
            
            fn build_included(&self) -> Option<Resources> {
                let mut included:Resources = vec![];
                $( included.append(&mut self.$has_one.to_resources()); )*
                $(
                    for model in &self.$has_many {
                        included.append(&mut model.to_resources());
                    }
                )*
                Some(included)
            }
        }
    );
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Dog {
    id: String,
    name: String,
    age: i32,
    main_flea: Flea,
    fleas: Vec<Flea>,
}
jsonapi_model!(Dog; "dog"; has one main_flea; has many fleas);
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Flea {
    id: String,
    name: String,
}
jsonapi_model!(Flea; "flea");
