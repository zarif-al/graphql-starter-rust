use crate::user::{generate_mock_users, seed_users};
use dotenv::dotenv;
use reset_db::reset_database;

mod reset_db;
mod user;

#[tokio::main]
async fn main() {
    // Load env from .env file
    dotenv().ok();

    // Setup logger
    // Update the `with_env_filter()` to get more or less logs
    tracing_subscriber::fmt().with_env_filter("seeder").init();

    // Before seeding the database we should reset the database
    // Cancel the seed operation if this step fails
    let reset_db = reset_database().await;
    match reset_db {
        Err(e) => {
            tracing::error!("Failed to reset database. Error: {}", e.to_string());
            panic!()
        }
        _ => {}
    }

    // Generate mock_users
    let mock_users = generate_mock_users(10);

    // Seed users
    let seed_users_results = seed_users(mock_users).await;
    match seed_users_results {
        Ok(()) => {
            tracing::info!("Users seed complete!");
        }
        Err(e) => {
            tracing::error!("Failed to seed users. Error: {}", e.to_string());
        }
    }
}
