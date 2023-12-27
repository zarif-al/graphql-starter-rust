use async_graphql::{InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::user::Entity as User;

use super::GraphQLUser;

#[derive(InputObject)]
pub struct FindOne {
    email: String,
}

#[derive(InputObject)]
pub struct FindMany {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    first: u32,
    after: Option<u32>,
}

pub async fn find_one_user(db: &DatabaseConnection, input: FindOne) -> Result<Option<GraphQLUser>> {
    let result = User::find_by_id(input.email).one(db).await?;

    match result {
        Some(user) => Ok(Some(user.into())),
        None => Ok(None),
    }
}
