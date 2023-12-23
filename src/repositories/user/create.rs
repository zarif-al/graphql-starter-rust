use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use tracing::info;

use crate::entities::user::{self, Model};

#[derive(InputObject)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
}

pub async fn create_user(db: &DatabaseConnection, input: CreateUser) -> Result<Model> {
    if input.first_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    if input.last_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    let new_user = user::ActiveModel {
        first_name: ActiveValue::Set(input.first_name),
        last_name: ActiveValue::Set(input.last_name),
        email: ActiveValue::Set("test@email.com".to_string()),
        ..Default::default()
    };

    let result = new_user.insert(db).await?;

    info!(
        "Inserted new user {} {}",
        result.first_name, result.last_name
    );

    Ok(result)
}

// #[cfg(test)]
// mod tests {
//     use crate::user::create::create_user;

//     use super::CreateUser;

//     #[test]
//     fn test_create_user() {
//         let username = String::from("Test User");

//         let input = CreateUser {
//             username: username.clone(),
//         };

//         let result = create_user(input).expect("User should be successfully created.");

//         assert_eq!(result.id, 1337);
//         assert_eq!(result.username, username);
//     }
// }
