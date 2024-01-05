pub fn generate_seed_posts(num: usize, user_ids: Vec<i32>) -> Vec<ActiveModel> {
    let mut seed_users: Vec<ActiveModel> = vec![];

    while mock_users.len() != num {
        mock_users.push(user::ActiveModel {
            first_name: Set(FirstName(EN).fake()),
            last_name: Set(LastName(EN).fake()),
            email: Set(SafeEmail(EN).fake()),
            ..Default::default()
        })
    }

    mock_users
}

async fn seed_posts() {}
