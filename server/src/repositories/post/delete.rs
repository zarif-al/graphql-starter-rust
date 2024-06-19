use crate::entities::prelude::{Post, User};
use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use tracing::error;

#[derive(InputObject)]
pub struct DeletePostInput {
    id: String,
    user_email: String,
}

pub async fn delete_post(db: &DatabaseConnection, input: DeletePostInput) -> Result<bool> {
    let result = Post::find_by_id(input.id)
        .find_also_related(User)
        .one(db)
        .await;

    // Extract post and user information from the result
    if let Ok(Some((post, optional_user))) = result {
        // Proceed with deletion logic if user is not NONE
        if let Some(user) = optional_user {
            if user.email == input.user_email {
                let result = post.delete(db).await;

                match result {
                    Ok(_) => return Ok(true),
                    Err(e) => {
                        error!("Post -> Delete: {}", e.to_string());
                        return Err(Error::new("500"));
                    }
                }
            }
        }
    }

    // Error: Post not found
    Err(Error::new("P100"))
}
