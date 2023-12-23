/// This module will contain all the graphQL mutations.
///
/// Ideally you should contain the core logic of a mutation in
/// a separate module and call those functions/methods from here.
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

use crate::repositories::user::{
    create::{create_user, CreateUser},
    GraphQLUser,
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
                let response: GraphQLUser = new_user.into();
                Ok(response)
            }
            Err(err) => Err(err),
        }
    }
}
