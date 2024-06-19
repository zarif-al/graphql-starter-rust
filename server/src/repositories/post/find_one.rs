use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::prelude::Post;

use super::GraphQLPost;

#[derive(InputObject)]
pub struct FindPostInput {
    id: String,
}

pub async fn find_post(
    db: &DatabaseConnection,
    input: FindPostInput,
) -> Result<Option<GraphQLPost>> {
    let results = Post::find_by_id(input.id).one(db).await;

    match results {
        Ok(post_option) => match post_option {
            Some(post) => Ok(Some(post.into())),
            None => Ok(None),
        },
        Err(e) => {
            tracing::error!("Source: Find post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
