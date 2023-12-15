/// This module will contain all the graphQL mutations.
///
/// Ideally you should contain the core logic of a mutation in
/// a separate module and call those functions/methods from here.
use async_graphql::{Object, Result};

use crate::user::create::{create_user, CreateUser};
use crate::user::definition::User;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    pub async fn create_user(&self, input: CreateUser) -> Result<User> {
        create_user(input)
    }
}
