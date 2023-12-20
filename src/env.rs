/// This module will read and return the values from the env file.
extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub struct Env {
    pub app_name: String,
    pub port: u16,
    pub db_url: String,
}

pub fn get_env() -> Env {
    dotenv().ok();

    let app_name = env::var("APP_NAME").unwrap_or("Rust GraphQL Service".to_string());

    let port = env::var("PORT")
        .unwrap_or("4000".to_string())
        .parse()
        .expect("PORT is expected to be parsable as u32");

    let db_url = env::var("DB_URL")
        .expect("The .env should have a database connection string provided as `DB_URL`.");

    Env {
        app_name,
        port,
        db_url,
    }
}
