use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::error;

use entity::definitions::{prelude::User, user::ActiveModel};
use uuid::Uuid;

use super::GraphQLUser;

#[derive(InputObject)]
pub struct UpdateUserInput {
    id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
}

pub async fn update_user(db: &DatabaseConnection, input: UpdateUserInput) -> Result<GraphQLUser> {
    // Get user model
    let result = User::find_by_id(input.id).one(db).await;

    // If we get an error log it and return 500
    if let Err(err) = result {
        error!("User -> Update: {}", err.to_string());
        return Err(Error::new("500"));
    }

    if let Ok(Some(user)) = result {
        let mut user: ActiveModel = user.into();

        if let Some(first_name) = input.first_name.filter(|s| !s.is_empty()) {
            user.first_name = Set(first_name);
        }

        if let Some(last_name) = input.last_name.filter(|s| !s.is_empty()) {
            user.last_name = Set(last_name);
        }

        if let Some(email) = input.email.filter(|s| !s.is_empty()) {
            user.email = Set(email);
        }

        let update_result = user.update(db).await;

        match update_result {
            Ok(user) => return Ok(user.into()),
            Err(e) => {
                error!("User -> Update: {}", e.to_string());
                return Err(Error::new("500"));
            }
        }
    }

    // If the Err() and Ok() block did not get triggered
    // then we can assume the user was not found.
    return Err(Error::new("U100"));
}
