use crate::entities::{post, prelude::Post};
use async_graphql::{Error, InputObject, Result};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use super::GraphQLPost;

#[derive(InputObject)]
pub struct FindUserPostsInput {
    user_id: i32,
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
        .filter(post::Column::UserId.eq(input.user_id))
        .paginate(db, page_size);

    let results = post_pages.fetch_page(input.page).await;

    match results {
        Ok(posts) => Ok(posts.into_iter().map(|post| post.into()).collect()),
        Err(e) => {
            tracing::error!("Source: Find user posts. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
