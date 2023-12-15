use async_graphql::Json;
use tracing::info;

use crate::user::definition::User;

pub fn create_user(username: String) -> Json<User> {
    info!("Creating user with argument: {}", username);

    Json(User {
        id: 1337,
        username: username,
    })
}
