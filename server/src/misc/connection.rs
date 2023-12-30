/// This module will contain return connection functions
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use crate::misc::env::get_env;

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let env = get_env();

    let mut opt = ConnectOptions::new(env.database_url);

    // Disable default logging
    opt.sqlx_logging(false);

    Database::connect(opt).await
}
