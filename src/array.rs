//! Defines trait and implementations that allow a `has many` relationship to be optional
use crate::model::JsonApiModel;

/// Trait which allows a `has many` relationship to be optional.
pub trait JsonApiArray<M> {
    fn get_models(&self) -> &[M];
    fn get_models_mut(&mut self) -> &mut [M];
}

impl<M: JsonApiModel> JsonApiArray<M> for Vec<M> {
    fn get_models(&self) -> &[M] { self }
    fn get_models_mut(&mut self) -> &mut [M] { self }
}

impl<M: JsonApiModel> JsonApiArray<M> for Option<Vec<M>> {
    fn get_models(&self) -> &[M] {
        self.as_ref()
            .map(|v| v.as_slice())
            .unwrap_or(&[][..])
    }

    fn get_models_mut(&mut self) -> &mut [M] {
        self.as_mut()
            .map(|v| v.as_mut_slice())
            .unwrap_or(&mut [][..])
    }
}
