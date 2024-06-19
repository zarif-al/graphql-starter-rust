use fake::faker::internet::raw::SafeEmail;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use sea_orm::DbErr;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use server::entities::user::{self, ActiveModel};
use server::misc::get_db_connection;
use tracing::info;
use uuid::Uuid;

pub fn generate_users_seed(num: usize) -> Vec<ActiveModel> {
    let mut users_seed: Vec<ActiveModel> = vec![];
    let mut i = 0;
    while i != num {
        users_seed.push(user::ActiveModel {
            first_name: Set(FirstName(EN).fake()),
            last_name: Set(LastName(EN).fake()),
            email: Set(SafeEmail(EN).fake()),
            id: Set(Uuid::new_v4().to_string()),
            ..Default::default()
        });
        i += 1;
    }

    users_seed
}

pub async fn seed_users(users: Vec<ActiveModel>) -> Result<(), DbErr> {
    // Get the connection and start a transaction
    let db = get_db_connection().await?;
    let transaction = db.begin().await?;

    // Insert with the transaction connection
    for user in users {
        user.insert(&transaction).await?;
    }

    // Commit it
    transaction.commit().await?;

    info!("Users seed complete!");

    Ok(())
}
