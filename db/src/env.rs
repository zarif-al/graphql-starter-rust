extern crate dotenv;

use std::env;

pub struct Env {
    pub app_name: String,
    pub port: u16,
    pub database_url: String,
}

pub fn get_env() -> Env {
    let app_name = env::var("APP_NAME").unwrap_or("Rust GraphQL Service".to_string());

    let port = env::var("PORT")
        .unwrap_or("4000".to_string())
        .parse()
        .expect("PORT is expected to be parsable as u32");

    let database_url = env::var("DATABASE_URL")
        .expect("The .env should have a database connection string provided as `DATABASE_URL`.");

    Env {
        app_name,
        port,
        database_url,
    }
}
