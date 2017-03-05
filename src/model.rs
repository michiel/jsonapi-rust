use api::Resource;

/// A trait for structs that can be represented as or built from a `Resource`
pub trait JsonApiModel {
    fn to_jsonapi_resource(&self) -> Resource;
    fn from_jsonapi_resource(resource: Resource) -> Self;
}
