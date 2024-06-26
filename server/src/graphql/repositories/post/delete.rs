use async_graphql::{Error, InputObject, Result};
use entity::definitions::prelude::{Post, User};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use tracing::error;
use uuid::Uuid;

#[derive(InputObject)]
pub struct DeletePostInput {
    id: Uuid,
    user_email: String,
}

pub async fn delete_post(db: &DatabaseConnection, input: DeletePostInput) -> Result<bool> {
    let post_result = Post::find_by_id(input.id)
        .find_also_related(User)
        .one(db)
        .await;

    // If we get an error Error and return 500
    if let Err(err) = post_result {
        error!("Post -> Delete: {}", err.to_string());
        return Err(Error::new("500"));
    }

    // Extract post and user information from the result
    if let Ok(Some((post, optional_user))) = post_result {
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

    // If the Err() and Ok() block are not triggered we can
    // safely assume the post was not found
    Err(Error::new("P100"))
}
