/// This module contains all the graphQL queries.
///
/// Ideally you should contain the core logic of a query in
/// a separate module and call those functions/methods from here.
use async_graphql::{Json, Object};

use crate::misc::{get_db_connection, responses::GeneralResponse};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn health_check(&self) -> Json<GeneralResponse> {
        let db = get_db_connection().await;

        match db {
            Ok(_) => Json(GeneralResponse {
                code: 200,
                error: None,
                message: Some("Database Connection Established!".to_string()),
            }),
            Err(e) => Json(GeneralResponse {
                code: 500,
                error: Some(e.to_string()),
                message: None,
            }),
        }
    }
}
