use crate::user::definition::User;
use async_graphql::{Error, InputObject, Result};
use tracing::info;

#[derive(InputObject)]
pub struct CreateUser {
    username: String,
}

pub fn create_user(input: CreateUser) -> Result<User> {
    if input.username.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    info!("Creating user with argument: {}", input.username);

    Ok(User {
        id: 1337,
        username: input.username,
    })
}
