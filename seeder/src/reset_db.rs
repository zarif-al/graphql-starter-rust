use db::get_db_connection;
use migration::Migrator;
use sea_orm_migration::prelude::*;

pub async fn reset_database() -> Result<(), DbErr> {
    let db = get_db_connection().await;

    match db {
        Ok(db) => {
            tracing::info!("Resetting database...");
            let migration_results = Migrator::refresh(&db).await;

            match migration_results {
                Ok(()) => {
                    tracing::info!("Database reset successfully!");

                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
