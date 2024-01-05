use crate::entities::{post, prelude::User};
use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use super::GraphQLPost;

#[derive(InputObject)]
pub struct CreatePostInput {
    user_id: i32,
    content: String,
}

pub async fn create_post(db: &DatabaseConnection, input: CreatePostInput) -> Result<GraphQLPost> {
    // Verify content is not empty string.
    if input.content.is_empty() {
        return Err(Error::new("Invalid post content"));
    }

    // Search for user with provided user_id.
    let user_search_result = User::find_by_id(input.user_id).one(db).await;

    match user_search_result {
        Ok(user_option) => {
            // If user exists create post otherwise throw error.
            if user_option.is_some() {
                let new_post = post::ActiveModel {
                    content: Set(input.content),
                    user_id: Set(input.user_id),
                    ..Default::default()
                };

                let result = new_post.insert(db).await;

                match result {
                    Ok(post_model) => Ok(post_model.into()),
                    Err(e) => {
                        tracing::error!("Source: Create Post. Message: {}", e.to_string());
                        Err(Error::new("500"))
                    }
                }
            } else {
                // Error: User does not exist
                return Err(Error::new("U100"));
            }
        }
        Err(e) => {
            tracing::error!("Source: Create Post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
