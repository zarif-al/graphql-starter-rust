use dotenv::dotenv;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    // Load env from .env file
    dotenv().ok();

    cli::run_cli(migration::Migrator).await;
}
