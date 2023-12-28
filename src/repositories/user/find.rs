use async_graphql::{InputObject, Result};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use crate::entities::user::{self, Entity as User};

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
    limit: u64,
    first: u64,
    after: Option<u64>,
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
    let mut cursor = user::Entity::find().cursor_by(user::Column::FirstName);

    cursor.after(input.after.unwrap_or(0));

    let results = cursor.first(input.limit).all(db).await?;

    Ok(results.into_iter().map())
}
