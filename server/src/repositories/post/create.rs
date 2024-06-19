use crate::entities::prelude::User;
use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use server::entities::{post, user};

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
                tracing::error!("Source: Create Post. Message: {}", e.to_string());
                return Err(Error::new("500"));
            }
        }
    }

    // Error: User does not exist
    return Err(Error::new("U100"));
}
