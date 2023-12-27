/// This module contains all the graphQL queries.
///
/// Ideally you should contain the core logic of a query in
/// a separate module and call those functions/methods from here.
use async_graphql::{Context, Json, Object, Result};
use sea_orm::DatabaseConnection;

use crate::{
    misc::responses::GeneralResponse,
    repositories::user::{
        find::{find_one_user, FindOne},
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

    pub async fn find_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FindOne,
    ) -> Result<Option<GraphQLUser>> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let user = find_one_user(&db, input).await?;

                match user {
                    Some(user) => Ok(Some(user)),
                    None => Ok(None),
                }
            }
            Err(err) => Err(err),
        }
    }
}
