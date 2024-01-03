use crate::entities::post;
use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

#[derive(InputObject)]
pub struct DeletePost {
    id: i32,
    user_id: i32,
}

pub async fn delete_post(db: &DatabaseConnection, input: DeletePost) -> Result<bool> {
    let result = post::Entity::find_by_id(input.id).one(db).await;

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
                    Err(Error::new("P104"))
                }
            }
            None => {
                // Error: Post not found
                Err(Error::new("P103"))
            }
        },
        Err(e) => {
            tracing::error!("Source: Delete post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
