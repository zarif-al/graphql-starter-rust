use crate::entities::{post, user};
use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use super::GraphQLPost;

#[derive(InputObject)]
pub struct CreatePost {
    user_id: i32,
    content: String,
}

pub async fn create_post(db: &DatabaseConnection, input: CreatePost) -> Result<GraphQLPost> {
    // Verify content is not empty string.
    if input.content.is_empty() {
        return Err(Error::new("Invalid post content"));
    }

    // Search for user with provided user_id.
    let user = user::Entity::find_by_id(input.user_id).one(db).await;

    match user {
        Ok(user) => {
            // If user exists create post otherwise throw error.
            if user.is_some() {
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
                return Err(Error::new("P100"));
            }
        }
        Err(e) => {
            tracing::error!("Source: Create Post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
