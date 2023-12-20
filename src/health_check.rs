use async_graphql::Json;
use sea_orm::Database;

use crate::{env::get_env, general_response::GeneralResponse};

pub async fn health_check() -> Json<GeneralResponse> {
    let connection_string = get_env().db_url;

    let db = Database::connect(connection_string).await;

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
