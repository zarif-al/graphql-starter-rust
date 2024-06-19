use async_graphql::{Error, InputObject, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error;

use crate::entities::user::{self, Entity as User};

use super::GraphQLUser;

#[derive(InputObject)]
pub struct FindUserInput {
    email: String,
}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    input: FindUserInput,
) -> Result<Option<GraphQLUser>> {
    let user_search_result = User::find()
        .filter(user::Column::Email.eq(input.email))
        .one(db)
        .await;

    match user_search_result {
        Ok(user_option) => match user_option {
            Some(user) => Ok(Some(user.into())),
            None => Ok(None),
        },
        Err(e) => {
            error!("User -> Find One: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{find_user_by_email, FindUserInput};
    use crate::entities::user::Model;
    use crate::repositories::user::GraphQLUser;
    use crate::repositories::user::_mock::get_mock_create_user_input;
    use crate::repositories::user::_mock::get_mock_graphql_user;
    use crate::repositories::user::_mock::get_mock_user_model;
    use sea_orm::DbErr;
    use sea_orm::Transaction;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[async_std::test]
    /// Test a successfull user find operation that returns some user
    async fn find_user_some() {
        // Create mock db user
        let create_user_input = get_mock_create_user_input();
        let mock_db_user: Model = get_mock_user_model(create_user_input.clone());

        // Instantiate expected results
        let expected_result: GraphQLUser = get_mock_graphql_user(mock_db_user.clone());

        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![mock_db_user.clone()]])
            .into_connection();

        // Run `find_user` function
        let result = find_user_by_email(
            &db,
            FindUserInput {
                email: mock_db_user.clone().email,
            },
        )
        .await
        .unwrap()
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
                r#"SELECT "user"."id", "user"."created_at", "user"."updated_at", "user"."first_name", "user"."last_name", "user"."email" FROM "user" WHERE "user"."email" = $1 LIMIT $2"#,
                [mock_db_user.email.into(), 1u64.into()]
            )]
        )
    }

    #[async_std::test]
    /// Test a user find operation that returns NONE
    async fn find_user_none() {
        // Create mock db user
        let create_user_input = get_mock_create_user_input();
        let mock_db_user: Model = get_mock_user_model(create_user_input.clone());

        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([Vec::<Model>::new()])
            .into_connection();

        // Run `find_user` function
        let result = find_user_by_email(
            &db,
            FindUserInput {
                email: mock_db_user.clone().email,
            },
        )
        .await
        .unwrap();

        // Check that the result is a NONE.
        assert!(result.is_none());

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "user"."id", "user"."created_at", "user"."updated_at", "user"."first_name", "user"."last_name", "user"."email" FROM "user" WHERE "user"."email" = $1 LIMIT $2"#,
                [mock_db_user.email.into(), 1u64.into()]
            )]
        )
    }

    #[async_std::test]
    /// Test a user find operation that fails
    async fn find_user_error() {
        // Create mock db user
        let create_user_input = get_mock_create_user_input();
        let mock_db_user: Model = get_mock_user_model(create_user_input.clone());

        // Create MockDatabase with mock query error
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::Custom("Unknown Error".to_string())])
            .into_connection();

        // Run `find_user` function
        let result = find_user_by_email(
            &db,
            FindUserInput {
                email: mock_db_user.clone().email,
            },
        )
        .await;

        // Check that the result is an error.
        assert!(result.is_err());

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "user"."id", "user"."created_at", "user"."updated_at", "user"."first_name", "user"."last_name", "user"."email" FROM "user" WHERE "user"."email" = $1 LIMIT $2"#,
                [mock_db_user.email.into(), 1u64.into()]
            )]
        )
    }
}
