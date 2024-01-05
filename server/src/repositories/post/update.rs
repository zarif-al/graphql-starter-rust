use crate::entities::post::{self, ActiveModel};
use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use super::GraphQLPost;

#[derive(InputObject)]
pub struct UpdatePostInput {
    id: i32,
    user_id: i32,
    content: String,
}

pub async fn update_post(db: &DatabaseConnection, input: UpdatePostInput) -> Result<GraphQLPost> {
    let result = post::Entity::find_by_id(input.id).one(db).await;

    match result {
        Ok(post_option) => match post_option {
            Some(post) => {
                // Check if post belongs to user
                if post.user_id == input.user_id {
                    let mut post: ActiveModel = post.into();

                    post.content = Set(input.content);

                    let result = post.update(db).await;

                    match result {
                        Ok(post) => Ok(post.into()),
                        Err(e) => {
                            tracing::error!("Source: Update post. Message: {}", e.to_string());
                            Err(Error::new("500"))
                        }
                    }
                } else {
                    // Error: Post does not belong to user
                    Err(Error::new("P101"))
                }
            }
            None => {
                // Error: Post not found
                Err(Error::new("P100"))
            }
        },
        Err(e) => {
            tracing::error!("Source: Update post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
