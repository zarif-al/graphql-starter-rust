use db::get_db_connection;
use entity::definitions::post::{self, ActiveModel};
use entity::definitions::user::ActiveModel as UserActiveModel;
use fake::faker::lorem::en::Paragraph;
use fake::Fake;
use sea_orm::{ActiveModelTrait, DbErr, Set, TransactionTrait};
use tracing::info;
use uuid::Uuid;

/**
 * This function will accept a `num` and a vector of user ids.
 * It will create a `num` amount of posts for each user.
 */
pub fn generate_posts_seed(num: usize, users: Vec<UserActiveModel>) -> Vec<ActiveModel> {
    let mut posts_seed: Vec<ActiveModel> = vec![];

    for user in users {
        let mut i = 0;
        while i != num {
            posts_seed.push(post::ActiveModel {
                content: Set(Paragraph(1..2).fake()),
                user_id: Set(user.id.clone().unwrap()),
                id: Set(Uuid::new_v4()),
                ..Default::default()
            });
            i += 1;
        }
    }

    posts_seed
}

pub async fn seed_posts(posts: Vec<ActiveModel>) -> Result<(), DbErr> {
    // Get the connection and start a transaction
    let db = get_db_connection().await?;
    let transaction = db.begin().await?;

    // Insert with the transaction connection
    for post in posts {
        post.insert(&transaction).await?;
    }

    // Commit it
    transaction.commit().await?;

    info!("Posts seed complete!");

    Ok(())
}
