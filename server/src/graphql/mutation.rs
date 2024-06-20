use async_graphql::{Context, Error, Object, Result};
use sea_orm::DatabaseConnection;

use crate::graphql::repositories::{
    post::{
        create::{create_post, CreatePostInput},
        delete::{delete_post, DeletePostInput},
        update::{update_post, UpdatePostInput},
        GraphQLPost,
    },
    user::{
        create::{create_user, CreateUserInput},
        delete::{delete_user, DeleteUserInput},
        update::{update_user, UpdateUserInput},
        GraphQLUser,
    },
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreateUserInput,
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

    pub async fn update_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: UpdateUserInput,
    ) -> Result<GraphQLUser> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let user = update_user(&db, input).await?;
                Ok(user)
            }
            Err(e) => {
                tracing::error!("Source: DB Connection. Message: {}", e.message);
                Err(Error::new("500"))
            }
        }
    }

    pub async fn delete_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: DeleteUserInput,
    ) -> Result<bool> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let result = delete_user(&db, input).await?;
                Ok(result)
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
        input: CreatePostInput,
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
        input: UpdatePostInput,
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

    pub async fn delete_post<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: DeletePostInput,
    ) -> Result<bool> {
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
