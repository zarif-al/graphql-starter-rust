use crate::entities::post;
use async_graphql::{Error, InputObject, Result};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};

use super::GraphQLPost;

#[derive(InputObject)]
pub struct FindPostsByUserIdInput {
    user_id: i32,
    limit: u64,
    page: u64,
}

pub async fn find_posts_by_user_id(
    db: &DatabaseConnection,
    input: FindPostsByUserIdInput,
) -> Result<Vec<GraphQLPost>> {
    // TODO: Look into find_also_related method to eager load users

    // Page size cannot be 0
    let page_size;
    if input.limit < 1 {
        page_size = 1
    } else {
        page_size = input.limit
    }

    // Create post pages pagination by user id
    let post_pages = post::Entity::find()
        .order_by_asc(post::Column::Id)
        .filter(post::Column::UserId.eq(input.user_id))
        .paginate(db, page_size);

    let post_search_results = post_pages.fetch_page(input.page).await;

    match post_search_results {
        Ok(posts) => Ok(posts
            .into_iter()
            .map(|post: post::Model| post.into())
            .collect()),
        Err(e) => {
            tracing::error!("Source: Find posts by user_id. Message: {}", e.to_string());
            Err(Error::new("500"))
        }
    }
}
