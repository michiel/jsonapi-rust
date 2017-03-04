use api::Resource;

pub trait JsonApiModel {
    fn to_jsonapi_resource(&self) -> Resource;
    fn from_jsonapi_resource(resource: Resource) -> Self;
}
