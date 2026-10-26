use sea_orm::entity::prelude::Uuid;
use serde::Serialize;

use crate::entity::team::Model;

#[derive(Debug, Clone, Serialize)]
/// Short reference to a team, embedded in other resources.
pub struct TeamRef {
    pub id: Uuid,
    pub name: String,
}

impl From<Model> for TeamRef {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}
