use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use server::entities::user;

#[derive(InputObject)]
pub struct DeleteUserInput {
    id: i32,
}

pub async fn delete_user(db: &DatabaseConnection, input: DeleteUserInput) -> Result<bool> {
    // Find the user
    let result = user::Entity::find_by_id(input.id).one(db).await;

    match result {
        Ok(user_option) => match user_option {
            // If user exists proceed to user deletion
            Some(user) => {
                let result = user.delete(db).await;

                match result {
                    Ok(_) => Ok(true),
                    Err(e) => {
                        tracing::error!("Source: Delete Error. Message: {}", e.to_string());
                        Err(Error::new("500"))
                    }
                }
            }
            None => Err(Error::new("U100")),
        },
        Err(e) => {
            tracing::error!("Source: Delete Error. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
