/// This module will contain all the graphQL mutations.
///
/// Ideally you should contain the core logic of a mutation in
/// a separate module and call those functions/methods from here.
use async_graphql::{Context, Error, Object, Result};
use sea_orm::DatabaseConnection;

use crate::repositories::{
    post::{
        create::{create_post, CreatePost},
        GraphQLPost,
    },
    user::{
        create::{create_user, CreateUser},
        GraphQLUser,
    },
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreateUser,
    ) -> Result<GraphQLUser> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let new_user = create_user(&db, input).await?;
                Ok(new_user)
            }
            Err(e) => {
                tracing::error!("Source: DB Connection. Message: {}", e.message);
                Err(Error::new("500"))
            }
        }
    }

    pub async fn create_post<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreatePost,
    ) -> Result<GraphQLPost> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let new_post = create_post(&db, input).await?;
                Ok(new_post)
            }
            Err(e) => {
                tracing::error!("Source: DB Connection. Message: {}", e.message);
                Err(Error::new("500"))
            }
        }
    }
}
