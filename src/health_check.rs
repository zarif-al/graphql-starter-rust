use async_graphql::Json;

use crate::general_response::GeneralResponse;

pub fn health_check() -> Json<GeneralResponse> {
    Json(GeneralResponse {
        code: 200,
        error: None,
        message: Some("All OK!".to_string()),
    })
}
