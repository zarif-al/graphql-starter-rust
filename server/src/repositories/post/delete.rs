use crate::entities::post;
use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use server::misc::responses::GeneralResponse;

#[derive(InputObject)]
pub struct DeletePost {
    user_id: i32,
    id: i32,
}

pub async fn delete_post(db: &DatabaseConnection, input: DeletePost) -> Result<GeneralResponse> {
    // Search for post with provided id.
    let post_search_result = post::Entity::find_by_id(input.id).one(db).await;

    match post_search_result {
        Ok(post_option) => {
            // If post exists then attempt to delete post otherwise throw error.
            match post_option {
                Some(post) => {
                    // Check if the user_id of the post matches the user_id in the input argument.
                    if post.user_id == input.user_id {
                        // Proceed to delete post
                        let result = post.delete(db).await;

                        match result {
                            Ok(_) => Ok(GeneralResponse {
                                code: 200,
                                error: None,
                                message: Some("Post has been deleted".to_string()),
                            }),
                            Err(e) => {
                                tracing::error!("Source: Delete Post. Message: {}", e.to_string());
                                Err(Error::new("500"))
                            }
                        }
                    } else {
                        // Error: Post does not belong to user
                        return Err(Error::new("P102"));
                    }
                }
                None => {
                    // Error: Post does not exist
                    return Err(Error::new("P101"));
                }
            }
        }
        Err(e) => {
            tracing::error!("Source: Delete Post. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
