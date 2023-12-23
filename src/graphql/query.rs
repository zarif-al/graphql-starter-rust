/// This module contains all the graphQL queries.
///
/// Ideally you should contain the core logic of a query in
/// a separate module and call those functions/methods from here.
use async_graphql::{Json, Object};

use crate::misc::responses::GeneralResponse;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn health_check(&self) -> Json<GeneralResponse> {
        Json(GeneralResponse {
            code: 200,
            error: None,
            message: Some("Application is up and running!".to_string()),
        })
    }
}
