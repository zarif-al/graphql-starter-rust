use super::misc::general_response::GeneralResponse;
use crate::graphql::repositories::{
    post::{
        find_many::{find_user_posts, FindUserPostsInput},
        find_one::{find_post, FindPostInput},
        GraphQLPost,
    },
    user::{
        find_many::{find_users, FindUsersInput},
        find_one::{find_user_by_email, FindUserInput},
        GraphQLUser,
    },
};
use async_graphql::{Context, Error, Json, Object, Result};
use sea_orm::DatabaseConnection;

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

    pub async fn find_user_posts<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FindUserPostsInput,
    ) -> Result<Vec<GraphQLPost>> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let posts = find_user_posts(&db, input).await?;

                Ok(posts)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn find_post<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: FindPostInput,
    ) -> Result<Option<GraphQLPost>> {
        let db_connection = ctx.data::<DatabaseConnection>();

        match db_connection {
            Ok(db) => {
                let post = find_post(&db, input).await?;

                Ok(post)
            }
            Err(err) => Err(err),
        }
    }
}
