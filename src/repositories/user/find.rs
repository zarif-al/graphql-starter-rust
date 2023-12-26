use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use tracing::info;

use crate::entities::user::{self};

use super::GraphQLUser;

#[derive(InputObject)]
pub struct FindOne {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
}

#[derive(InputObject)]
pub struct FindMany {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    first: u32,
    after: Option<u32>,
}

pub async fn find_one_user(db: &DatabaseConnection, input: FindOne) -> Result<GraphQLUser> {
    if input.first_name.is_none() && input.last_name.is_none() && input.email.is_none() {
        return Err(Error::new("Please provide "));
    }

    if input.last_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    let result = new_user.insert(db).await?;

    info!(
        "Inserted new user {} {}",
        result.first_name, result.last_name
    );

    Ok(result.into())
}
