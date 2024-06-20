use async_graphql::{Error, InputObject, Result};
use entity::definitions::{post, prelude::Post, prelude::User, user};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use tracing::error;

use super::GraphQLPost;

#[derive(InputObject)]
pub struct FindUserPostsInput {
    user_email: String,
    limit: u64,
    page: u64,
}

pub async fn find_user_posts(
    db: &DatabaseConnection,
    input: FindUserPostsInput,
) -> Result<Vec<GraphQLPost>> {
    let page_size;

    if input.limit == 0 {
        page_size = 1
    } else {
        page_size = input.limit
    }

    // Offset based pagination
    let post_pages = Post::find()
        .order_by_asc(post::Column::UpdatedAt)
        .find_also_related(User)
        .filter(user::Column::Email.eq(input.user_email))
        .paginate(db, page_size);

    let results = post_pages.fetch_page(input.page).await;

    match results {
        Ok(posts) => Ok(posts.into_iter().map(|(post, _)| post.into()).collect()),
        Err(e) => {
            error!("Post -> Find User Posts: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
