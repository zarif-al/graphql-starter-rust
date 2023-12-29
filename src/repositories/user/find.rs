use async_graphql::{InputObject, Result};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};

use crate::entities::user::{self, Entity as User};

use super::GraphQLUser;

#[derive(InputObject)]
pub struct FindUserInput {
    email: String,
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

#[derive(InputObject)]
pub struct FindUsersInput {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    limit: u64,
    page: u64,
}

pub async fn find_users(
    db: &DatabaseConnection,
    input: FindUsersInput,
) -> Result<Vec<GraphQLUser>> {
    // Offset based pagination
    let paginate = user::Entity::find()
        .order_by_asc(user::Column::FirstName)
        .paginate(db, input.limit);

    let user_pages = paginate;

    let users = user_pages.fetch_page(input.page).await?;

    Ok(users.into_iter().map(|user| user.into()).collect())

    // Cursor based pagination
    // let mut cursor = user::Entity::find().cursor_by(user::Column::FirstName);
    // if let Some(after) = input.after {
    //     cursor.after(after);
    // }
    // let results = cursor.first(input.limit).all(db).await?;
    // Ok(results.into_iter().map(|user| user.into()).collect())
}
