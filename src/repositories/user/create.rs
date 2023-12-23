use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, ActiveValue};
use tracing::info;

use crate::{
    entities::user::{self, Model},
    misc::get_db_connection,
};

#[derive(InputObject)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
}

pub async fn create_user(input: CreateUser) -> Result<Model> {
    if input.first_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    if input.last_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    info!("Inserting new user to database");

    let new_user = user::ActiveModel {
        first_name: ActiveValue::Set(input.first_name),
        last_name: ActiveValue::Set(input.last_name),
        email: ActiveValue::Set("test@email.com".to_string()),
        ..Default::default()
    };

    let db = get_db_connection().await;

    match db {
        Ok(db) => {
            let result = new_user.insert(&db).await?;

            Ok(result)
        }
        Err(err) => Err(err.into()),
    }
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
