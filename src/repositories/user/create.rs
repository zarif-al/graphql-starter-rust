use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use tracing::info;

use crate::entities::user::{self};

use super::GraphQLUser;

#[derive(InputObject)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
}

pub async fn create_user(db: &DatabaseConnection, input: CreateUser) -> Result<GraphQLUser> {
    if input.first_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    if input.last_name.is_empty() {
        return Err(Error::new("Invalid username value"));
    }

    let new_user = user::ActiveModel {
        first_name: ActiveValue::Set(input.first_name),
        last_name: ActiveValue::Set(input.last_name),
        email: ActiveValue::Set(input.email),
        ..Default::default()
    };

    let result = new_user.insert(db).await?;

    info!(
        "Inserted new user {} {}",
        result.first_name, result.last_name
    );

    Ok(result.into())
}

// So far I have not found a reasonable testing strategy. I will resume this later.

// #[cfg(test)]
// mod tests {
//     use super::create_user;
//     use super::CreateUser;
//     use crate::entities::user;
//     use crate::repositories::user::GraphQLUser;
//     use chrono::Utc;
//     use sea_orm::DbErr;
//     use sea_orm::{DatabaseBackend, MockDatabase};

//     #[async_std::test]
//     async fn test_create_user_successfull() {
//         let creation_time = Utc::now();
//         let updated_time = Utc::now();
//         let first_name = "Test".to_string();
//         let last_name = "User".to_string();
//         let email = "test@email.com".to_string();

//         let expected_result: GraphQLUser = GraphQLUser {
//             created_at: creation_time.timestamp(),
//             updated_at: updated_time.timestamp(),
//             first_name: first_name.clone(),
//             last_name: last_name.clone(),
//             email: email.clone(),
//         };

//         // Create MockDatabase with mock query results
//         let db = MockDatabase::new(DatabaseBackend::Postgres)
//             .append_query_results([vec![user::Model {
//                 first_name: first_name.clone(),
//                 last_name: last_name.clone(),
//                 email: email.clone(),
//                 created_at: creation_time.into(),
//                 updated_at: updated_time.into(),
//             }]])
//             .into_connection();

//         let result = create_user(
//             &db,
//             CreateUser {
//                 first_name: first_name.clone(),
//                 last_name: last_name.clone(),
//                 email: email.clone(),
//             },
//         )
//         .await
//         .unwrap();

//         // Check result against expected result
//         assert_eq!(result.created_at, expected_result.created_at);
//         assert_eq!(result.updated_at, expected_result.updated_at);
//         assert_eq!(result.first_name, expected_result.first_name);
//         assert_eq!(result.last_name, expected_result.last_name);
//         assert_eq!(result.email, expected_result.email);
//     }
// }
