use crate::user::{generate_users_seed, seed_users};
use dotenv::dotenv;
use post::{generate_posts_seed, seed_posts};
use reset_db::reset_database;

mod post;
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

    // Generate users seed
    let users = generate_users_seed(10);

    // Seed users
    let seed_users_results = seed_users(users.clone()).await;
    match seed_users_results {
        Ok(()) => {
            tracing::info!("Users seed complete!");
        }
        Err(e) => {
            tracing::error!("Failed to seed users. Error: {}", e.to_string());
        }
    }

    // Generate posts seed
    let posts = generate_posts_seed(10, users.clone());

    // Seed posts
    let seed_posts_results = seed_posts(posts).await;
    match seed_posts_results {
        Ok(()) => {
            tracing::info!("Posts seed complete!");
        }
        Err(e) => {
            tracing::error!("Failed to seed posts. Error: {}", e.to_string());
        }
    }
}
