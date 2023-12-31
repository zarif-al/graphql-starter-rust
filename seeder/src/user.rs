use chrono::Utc;
use fake::faker::internet::raw::SafeEmail;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;
use sea_orm::DbErr;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use server::entities::user::ActiveModel;
use server::entities::user::{self};
use server::misc::get_db_connection;

pub fn generate_mock_users(num: usize) -> Vec<ActiveModel> {
    let mut mock_users: Vec<ActiveModel> = vec![];

    while mock_users.len() != num {
        mock_users.push(user::ActiveModel {
            first_name: Set(FirstName(EN).fake()),
            last_name: Set(LastName(EN).fake()),
            email: Set(SafeEmail(EN).fake()),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
        })
    }

    mock_users
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

    Ok(())
}
