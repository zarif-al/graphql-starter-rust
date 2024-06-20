#[cfg(test)]
mod _mock;
pub mod create;
pub mod delete;
pub mod find_many;
pub mod find_one;
pub mod update;

use entity::definitions::user::Model as UserModel;

use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct GraphQLUser {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl From<UserModel> for GraphQLUser {
    fn from(value: UserModel) -> Self {
        GraphQLUser {
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.timestamp(),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            id: value.id,
        }
    }
}
