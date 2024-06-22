use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use tracing::error;

use entity::definitions::prelude::User;
use uuid::Uuid;

#[derive(InputObject)]
pub struct DeleteUserInput {
    id: Uuid,
}

pub async fn delete_user(db: &DatabaseConnection, input: DeleteUserInput) -> Result<bool> {
    // Find the user
    let result = User::find_by_id(input.id).one(db).await;

    // If we get an error log it and return 500
    if let Err(err) = result {
        error!("Post -> Delete: {}", err.to_string());
        return Err(Error::new("500"));
    }

    // Extract the user and proceed to deletion
    if let Ok(Some(user)) = result {
        let result = user.delete(db).await;

        match result {
            Ok(_) => {
                return Ok(true);
            }
            Err(e) => {
                error!("Post -> Delete: {}", e.to_string());
                return Err(Error::new("500"));
            }
        }
    }

    // If the Ok() and Err() block did not get triggered then we
    // can assume the user was not found
    return Err(Error::new("U100"));
}
