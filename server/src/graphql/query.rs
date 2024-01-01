/// This module contains all the graphQL queries.
///
/// Ideally you should contain the core logic of a query in
/// a separate module and call those functions/methods from here.
use async_graphql::{Context, Error, Json, Object, Result};
use sea_orm::DatabaseConnection;

use crate::{
    misc::responses::GeneralResponse,
    repositories::user::{
        find_many::{find_users, FindUsersInput},
        find_one::{find_user_by_email, FindUserInput},
        GraphQLUser,
    },
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn health_check(&self) -> Json<GeneralResponse> {
        Json(GeneralResponse {
            code: 200,
            error: None,
            message: Some("Application is up and running!".to_string()),
        })
    }

    pub async fn find_user_by_email<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FindUserInput,
    ) -> Result<Option<GraphQLUser>> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let user = find_user_by_email(&db, input).await?;

                match user {
                    Some(user) => Ok(Some(user)),
                    None => Ok(None),
                }
            }
            Err(e) => {
                tracing::error!("Source: DB Connection. Message: {}", e.message);
                Err(Error::new("500"))
            }
        }
    }

    pub async fn find_users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FindUsersInput,
    ) -> Result<Vec<GraphQLUser>> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let users = find_users(&db, input).await?;

                Ok(users)
            }
            Err(e) => {
                tracing::error!("Source: DB Connection. Message: {}", e.message);
                Err(Error::new("500"))
            }
        }
    }
}
