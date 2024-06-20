use std::io;

use dotenv::dotenv;
use migration::MigratorTrait;

use server::misc::get_db_connection;

#[async_std::main]
async fn main() {
    // Load env from .env file
    dotenv().ok();

    let db_connection_result = get_db_connection().await;

    if let Err(err) = db_connection_result {
        eprintln!("Failed to connect to database. Error: {}", err.to_string());
        return;
    }

    // We should be able to safely unwrap db_connection_result here
    let db = db_connection_result.unwrap();

    loop {
        let mut input = String::new();

        println!("\nMigration Options:");
        println!("1. Fresh: Drop all tables from the database and reapply all migrations");
        println!("2. Refresh: Rollback all applied migrations and reapply them.");
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
                // Fresh Migration
                1 => {
                    let result = migration::Migrator::fresh(&db).await;

                    if let Err(error) = result {
                        eprintln!(
                            "Failed to perform fresh migration. Error: {}",
                            error.to_string()
                        );
                        eprintln!("Please try again.");
                        continue;
                    } else {
                        println!("Successfully ran fresh migrations in the database!");
                    }
                }
                // Refresh Migrations
                2 => {
                    let result = migration::Migrator::refresh(&db).await;

                    if let Err(error) = result {
                        eprintln!("Failed to refresh migrations. Error: {}", error.to_string());
                        eprintln!("Please try again.");
                        continue;
                    } else {
                        println!("Successfully refreshed the migrations in the database!");
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
