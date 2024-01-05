use crate::entities::prelude::Post;
use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

#[derive(InputObject)]
pub struct DeletePostInput {
    id: i32,
    user_id: i32,
}

pub async fn delete_post(db: &DatabaseConnection, input: DeletePostInput) -> Result<bool> {
    let result = Post::find_by_id(input.id).one(db).await;

    match result {
        Ok(post_option) => match post_option {
            Some(post) => {
                // Check if post belongs to user
                if post.user_id == input.user_id {
                    let result = post.delete(db).await;

                    match result {
                        Ok(_) => Ok(true),
                        Err(e) => {
                            tracing::error!("Source: Delete post. Message: {}", e.to_string());
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
            tracing::error!("Source: Delete post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
