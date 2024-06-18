use crate::user::{generate_users_seed, seed_users};
use dotenv::dotenv;
use post::{generate_posts_seed, seed_posts};
use reset_db::reset_database;
use std::io;

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

    loop {
        let mut input = String::new();

        println!("\nSeeder Options:");
        println!("1. Reset database: This will remove all of the data entry in the database.");
        println!("2. Re-seed database: This will clear the database and re-seed all the entities.");
        println!("3. Exit");

        println!("\nYour Selection: ");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!();
            }
            Err(e) => {
                eprintln!("\nFailed to read input. Error: {}", e.to_string());
                continue;
            }
        }

        if let Ok(input_value) = input.trim().parse::<i32>() {
            match input_value {
                // Reset database
                1 => {
                    let reset_db = reset_database().await;

                    if let Err(error) = reset_db {
                        eprintln!("Failed to reset the database. Error: {}", error.to_string());
                        eprintln!("Please try again.");
                        continue;
                    }
                }
                // Re-seed all entities
                2 => {
                    let reset_db = reset_database().await;

                    if let Err(error) = reset_db {
                        eprintln!("Failed to reset the database. Error: {}", error.to_string());
                        eprintln!("Please try again.");
                        continue;
                    }

                    // Generate users seed
                    let users = generate_users_seed(10);

                    // Seed users
                    let seed_users_results = seed_users(users.clone()).await;

                    if let Err(e) = seed_users_results {
                        eprintln!("Failed to seed users. Error: {}", e.to_string());
                        eprintln!("Please try again.");
                    }

                    // Generate posts seed
                    let posts = generate_posts_seed(10, users.clone());

                    // Seed posts
                    let seed_posts_results = seed_posts(posts).await;

                    if let Err(e) = seed_posts_results {
                        eprintln!("Failed to seed posts. Error: {}", e.to_string());
                        eprintln!("Please try again.");
                    }
                }
                3 => {
                    break;
                }
                _ => {
                    eprintln!("Please input a number within the provided range.");
                    continue;
                }
            }
        } else {
            eprintln!("Please input a valid number.");
            continue;
        }
    }
}
