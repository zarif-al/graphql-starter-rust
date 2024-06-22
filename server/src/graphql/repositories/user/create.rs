use async_graphql::{Error, InputObject, Result};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use tracing::error;
use uuid::Uuid;

use entity::definitions::user;

use super::GraphQLUser;

#[derive(InputObject, Clone)]
pub struct CreateUserInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn create_user(db: &DatabaseConnection, input: CreateUserInput) -> Result<GraphQLUser> {
    // Error: Invalid input
    if input.first_name.is_empty() || input.last_name.is_empty() || input.email.is_empty() {
        return Err(Error::new("I100"));
    }

    let new_user = user::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        first_name: ActiveValue::Set(input.first_name),
        last_name: ActiveValue::Set(input.last_name),
        email: ActiveValue::Set(input.email),
        ..Default::default()
    };

    let result = new_user.insert(db).await;

    match result {
        Ok(user_model) => Ok(user_model.into()),
        Err(e) => {
            error!("User -> Create: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::create_user;
    use crate::graphql::repositories::user::GraphQLUser;
    use crate::graphql::repositories::user::_mock::get_mock_create_user_input;
    use crate::graphql::repositories::user::_mock::get_mock_graphql_user;
    use crate::graphql::repositories::user::_mock::get_mock_user_model;
    use entity::definitions::user::Model;
    use sea_orm::DbErr;
    use sea_orm::Transaction;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[async_std::test]
    /// Test a successfull user creation operation
    async fn create_user_success() {
        // Instantiate create user input
        let create_user_input = get_mock_create_user_input();

        // Instantiate db user
        let mock_db_user: Model = get_mock_user_model(create_user_input.clone());

        // Instantiate expected results
        let expected_result: GraphQLUser = get_mock_graphql_user(mock_db_user.clone());

        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![mock_db_user]])
            .into_connection();

        // Run `create_user` function
        let result = create_user(&db, create_user_input.clone()).await.unwrap();

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
                r#"INSERT INTO "user" ("first_name", "last_name", "email") VALUES ($1, $2, $3) RETURNING "id", "created_at", "updated_at", "first_name", "last_name", "email""#,
                [
                    create_user_input.first_name.into(),
                    create_user_input.last_name.into(),
                    create_user_input.email.into()
                ]
            )]
        )
    }

    #[async_std::test]
    /// Test a failed user creation operation
    async fn create_user_failure() {
        // Instantiate create user input
        let create_user_input = get_mock_create_user_input();

        // Create MockDatabase with mock query error
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotInserted])
            .into_connection();

        // Run `create_user` function
        let result = create_user(&db, create_user_input.clone()).await;

        // Check if the operation results in an error
        assert!(result.is_err());

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "user" ("first_name", "last_name", "email") VALUES ($1, $2, $3) RETURNING "id", "created_at", "updated_at", "first_name", "last_name", "email""#,
                [
                    create_user_input.first_name.into(),
                    create_user_input.last_name.into(),
                    create_user_input.email.into()
                ]
            )]
        )
    }
}
