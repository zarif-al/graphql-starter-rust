use async_graphql::{Error, InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use tracing::error;

use entity::definitions::{prelude::User, user};

use super::GraphQLUser;

#[derive(InputObject)]
pub struct FindUsersInput {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    limit: u64,
    page: u64,
}

pub async fn find_users(
    db: &DatabaseConnection,
    input: FindUsersInput,
) -> Result<Vec<GraphQLUser>> {
    // Page size cannot be 0
    let page_size;
    if input.limit < 1 {
        page_size = 1
    } else {
        page_size = input.limit
    }

    let user_pages = User::find()
        .order_by_asc(user::Column::FirstName)
        .paginate(db, page_size);

    let results = user_pages.fetch_page(input.page).await;

    match results {
        Ok(users) => Ok(users.into_iter().map(|user| user.into()).collect()),
        Err(e) => {
            error!("User -> Find Many: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{find_users, FindUsersInput};
    use crate::graphql::repositories::user::_mock::get_mock_create_user_input;
    use crate::graphql::repositories::user::_mock::get_mock_user_model;
    use entity::definitions::user::Model;
    use sea_orm::DbErr;
    use sea_orm::Transaction;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[async_std::test]
    /// Test a successfull user find operation that returns a vector of users
    async fn find_users_success() {
        // Instantiate find users input argument
        let input_argument: FindUsersInput = FindUsersInput {
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            email: Some("test@email.com".to_string()),
            limit: 0,
            page: 0,
        };

        // Create mock db users
        let mut mock_db_users: Vec<Model> = vec![];
        while mock_db_users.len() < 10 {
            let create_user_input = get_mock_create_user_input();
            let mock_db_user: Model = get_mock_user_model(create_user_input.clone());

            mock_db_users.push(mock_db_user);
        }

        // Create MockDatabase with mock query results
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([mock_db_users.clone()])
            .into_connection();

        // Run `find_users` function
        let result = find_users(&db, input_argument).await.unwrap();

        // Check result against expected result
        assert_eq!(result.len(), mock_db_users.len());

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "user"."id", "user"."created_at", "user"."updated_at", "user"."first_name", "user"."last_name", "user"."email" FROM "user" ORDER BY "user"."first_name" ASC LIMIT $1 OFFSET $2"#,
                [1u64.into(), 0u64.into()]
            )]
        )
    }

    #[async_std::test]
    /// Test a users find operation that fails
    async fn find_users_error() {
        // Instantiate find users input argument
        let input_argument: FindUsersInput = FindUsersInput {
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            email: Some("test@email.com".to_string()),
            limit: 0,
            page: 0,
        };

        // Create MockDatabase with mock query error
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::Custom("Unknown Error".to_string())])
            .into_connection();

        // Run `find_users` function
        let result = find_users(&db, input_argument).await;

        // Check that the result is an error.
        assert!(result.is_err());

        // Check the transaction log to make sure the correct SQL operation is being run
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "user"."id", "user"."created_at", "user"."updated_at", "user"."first_name", "user"."last_name", "user"."email" FROM "user" ORDER BY "user"."first_name" ASC LIMIT $1 OFFSET $2"#,
                [1u64.into(), 0u64.into()]
            )]
        )
    }
}
