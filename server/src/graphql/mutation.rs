use async_graphql::{Context, Error, Object, Result};
use sea_orm::DatabaseConnection;

use crate::repositories::{
    post::{
        create::{create_post, CreatePost},
        delete::{delete_post, DeletePost},
        update::{update_post, UpdatePost},
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

    pub async fn update_post<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: UpdatePost,
    ) -> Result<GraphQLPost> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let result = update_post(&db, input).await?;
                Ok(result)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn delete_post<'ctx>(&self, ctx: &Context<'ctx>, input: DeletePost) -> Result<bool> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let result = delete_post(&db, input).await?;
                Ok(result)
            }
            Err(err) => Err(err),
        }
    }
}
