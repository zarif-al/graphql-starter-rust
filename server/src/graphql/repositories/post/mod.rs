pub mod create;
pub mod delete;
pub mod find_many;
pub mod find_one;
pub mod update;

use async_graphql::SimpleObject;
use entity::definitions::post::Model as PostModel;
use uuid::Uuid;

#[derive(SimpleObject)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct GraphQLPost {
    pub id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub content: String,
    pub user_id: Uuid,
}

impl From<PostModel> for GraphQLPost {
    fn from(value: PostModel) -> Self {
        GraphQLPost {
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.timestamp(),
            content: value.content,
            user_id: value.user_id,
            id: value.id,
        }
    }
}
