use async_graphql::{Error, InputObject, Result};
use entity::definitions::{post, prelude::User, user};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::error;

use super::GraphQLPost;

#[derive(InputObject)]
pub struct CreatePostInput {
    user_email: String,
    content: String,
}

pub async fn create_post(db: &DatabaseConnection, input: CreatePostInput) -> Result<GraphQLPost> {
    // Verify content is not empty string.
    if input.content.is_empty() {
        return Err(Error::new("Invalid post content"));
    }

    // Search for user with provided user_id.
    let user_result = User::find()
        .filter(user::Column::Email.eq(input.user_email))
        .one(db)
        .await;

    // If we get an error Error and return 500
    if let Err(err) = user_result {
        error!("Post -> Create: {}", err.to_string());
        return Err(Error::new("500"));
    }

    // Proceed with post creation logic if user is found
    if let Ok(Some(user)) = user_result {
        let new_post = post::ActiveModel {
            content: Set(input.content),
            user_id: Set(user.id),
            ..Default::default()
        };

        let result = new_post.insert(db).await;

        match result {
            Ok(post_model) => {
                return Ok(post_model.into());
            }
            Err(e) => {
                error!("Post -> Create: {}", e.to_string());
                return Err(Error::new("500"));
            }
        }
    }

    // If the Ok() and Err() blocks don't get triggerd then
    // we can safely assume the user does not exist and return a user
    // not found
    return Err(Error::new("U100"));
}
