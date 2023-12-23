/// This module will contain all the graphQL mutations.
///
/// Ideally you should contain the core logic of a mutation in
/// a separate module and call those functions/methods from here.
use async_graphql::{Object, Result};

use crate::repositories::user::{
    create::{create_user, CreateUser},
    definition::GraphQLUser,
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user(&self, input: CreateUser) -> Result<GraphQLUser> {
        let new_user = create_user(input).await?;
        let response: GraphQLUser = new_user.into();
        Ok(response)
    }
}
