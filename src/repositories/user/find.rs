use async_graphql::{InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::user::Entity as User;

use super::GraphQLUser;

#[derive(InputObject)]
pub struct FindUserInput {
    email: String,
}

#[derive(InputObject)]
pub struct FindUsersInput {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    first: u32,
    after: Option<u32>,
}

pub async fn find_user(
    db: &DatabaseConnection,
    input: FindUserInput,
) -> Result<Option<GraphQLUser>> {
    let result = User::find_by_id(input.email).one(db).await?;

    match result {
        Some(user) => Ok(Some(user.into())),
        None => Ok(None),
    }
}

pub async fn find_users(
    db: &DatabaseConnection,
    input: FindUsersInput,
) -> Result<Vec<GraphQLUser>> {
    Ok(vec![])
}
