#[cfg(test)]
mod _mock;
pub mod create;
pub mod delete;
pub mod find_many;
pub mod find_one;
pub mod update;

use crate::entities;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct GraphQLPost {
    pub id: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub content: String,
    pub user_id: i32,
}

impl From<entities::post::Model> for GraphQLPost {
    fn from(value: entities::post::Model) -> Self {
        GraphQLPost {
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.timestamp(),
            content: value.content,
            user_id: value.user_id,
            id: value.id,
        }
    }
}
