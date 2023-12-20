/// This module contains all the graphQL queries.
///
/// Ideally you should contain the core logic of a query in
/// a separate module and call those functions/methods from here.
use async_graphql::{Json, Object};

use crate::general_response::GeneralResponse;
use crate::health_check::health_check;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn health_check(&self) -> Json<GeneralResponse> {
        health_check().await
    }
}
