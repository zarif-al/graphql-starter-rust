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

#[cfg(test)]
mod tests {
    use super::create_user;
    use super::CreateUser;
    use crate::entities::user;
    use crate::entities::user::Model;
    use crate::repositories::user::GraphQLUser;
    use chrono::Utc;
    use sea_orm::DbErr;
    use sea_orm::Transaction;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[async_std::test]
    /// Test a successfull user creation operation
    async fn create_user_success() {
        // Instantiate `user` properties
        let first_name = "Test".to_string();
        let last_name = "User".to_string();
        let email = "test@gmail.com".to_string();

        // Instantiate db user
        let mock_db_user: Model = user::Model {
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            email: email.clone(),
        };

        // Instantiate expected results
        let expected_result: GraphQLUser = GraphQLUser {
            created_at: mock_db_user.clone().created_at.timestamp(),
            updated_at: mock_db_user.clone().updated_at.timestamp(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            email: email.clone(),
        };

        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![mock_db_user]])
            .into_connection();

        // Run `create_user` function
        let result = create_user(
            &db,
            CreateUser {
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                email: email.clone(),
            },
        )
        .await
        .unwrap();

        // Check result against expected result
        assert_eq!(result.created_at, expected_result.created_at);
        assert_eq!(result.updated_at, expected_result.updated_at);
        assert_eq!(result.first_name, expected_result.first_name);
        assert_eq!(result.last_name, expected_result.last_name);
        assert_eq!(result.email, expected_result.email);

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "user" ("first_name", "last_name", "email") VALUES ($1, $2, $3) RETURNING "created_at", "updated_at", "first_name", "last_name", "email""#,
                [
                    first_name.clone().into(),
                    last_name.clone().into(),
                    email.clone().into()
                ]
            )]
        )
    }

    #[async_std::test]
    /// Test a failed user creation operation
    async fn create_user_failure() {
        // Instantiate `user` properties
        let first_name = "Test".to_string();
        let last_name = "User".to_string();
        let email = "test@gmail.com".to_string();

        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotInserted])
            .into_connection();

        // Run `create_user` function
        let result = create_user(
            &db,
            CreateUser {
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                email: email.clone(),
            },
        )
        .await;

        // Check if the operation results in an error
        assert!(result.is_err());

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "user" ("first_name", "last_name", "email") VALUES ($1, $2, $3) RETURNING "created_at", "updated_at", "first_name", "last_name", "email""#,
                [
                    first_name.clone().into(),
                    last_name.clone().into(),
                    email.clone().into()
                ]
            )]
        )
    }
}
