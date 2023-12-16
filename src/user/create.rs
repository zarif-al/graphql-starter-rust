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

#[cfg(test)]
mod tests {
    use crate::user::create::create_user;

    use super::CreateUser;

    #[test]
    fn test_create_user() {
        let username = String::from("Test User");

        let input = CreateUser {
            username: username.clone(),
        };

        let result = create_user(input).expect("User should be successfully created.");

        assert_eq!(result.id, 1337);
        assert_eq!(result.username, username);
    }
}
