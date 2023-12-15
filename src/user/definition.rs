use async_graphql::SimpleObject;
use serde::Serialize;

#[derive(Serialize, SimpleObject)]
pub struct User {
    pub id: u64,
    pub username: String,
}
