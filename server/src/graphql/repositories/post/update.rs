use async_graphql::{Error, InputObject, Result};
use entity::definitions::{
    post::{self, ActiveModel},
    prelude::User,
    user,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::error;

use super::GraphQLPost;

#[derive(InputObject)]
pub struct UpdatePostInput {
    id: String,
    user_email: String,
    content: String,
}

pub async fn update_post(db: &DatabaseConnection, input: UpdatePostInput) -> Result<GraphQLPost> {
    let result = post::Entity::find()
        .find_also_related(User)
        .filter(user::Column::Email.eq(input.user_email))
        .one(db)
        .await;

    // If we get an error return 500
    if let Err(err) = result {
        error!("Post -> Update: {}", err.to_string());
        return Err(Error::new("500"));
    }

    // Extract post from result and proceed to update logic
    if let Ok(Some((post, _))) = result {
        let mut post: ActiveModel = post.into();

        post.content = Set(input.content);

        let result = post.update(db).await;

        match result {
            Ok(post) => return Ok(post.into()),
            Err(e) => {
                error!("Post -> Update: {}", e.to_string());
                return Err(Error::new("500"));
            }
        }
    }

    // If the Err() and Ok() block did not get triggered
    // then we can safely assume the post was not found.
    return Err(Error::new("P100"));
}
