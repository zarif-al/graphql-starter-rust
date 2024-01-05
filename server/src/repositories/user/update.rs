use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::entities::{prelude::User, user::ActiveModel};

use super::GraphQLUser;

#[derive(InputObject)]
pub struct UpdateUserInput {
    id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
}

pub async fn update_user(db: &DatabaseConnection, input: UpdateUserInput) -> Result<GraphQLUser> {
    // Validate inputs
    if input.first_name.is_none() && input.last_name.is_none() {
        return Err(Error::new("I100"));
    }

    // Get user model
    let result = User::find_by_id(input.id).one(db).await;

    match result {
        Ok(user_option) => match user_option {
            Some(user) => {
                let mut user: ActiveModel = user.into();

                if let Some(first_name) = input.first_name {
                    user.first_name = Set(first_name);
                }

                if let Some(last_name) = input.last_name {
                    user.last_name = Set(last_name);
                }

                let update_result = user.update(db).await;

                match update_result {
                    Ok(user) => Ok(user.into()),
                    Err(e) => {
                        tracing::error!("Source: Update user. Message: {}", e.to_string());
                        Err(Error::new("500"))
                    }
                }
            }
            None => Err(Error::new("U100")),
        },
        Err(e) => {
            tracing::error!("Source: Update user. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
