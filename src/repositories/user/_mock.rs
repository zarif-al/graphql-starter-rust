/// This module will export mock structs for testing and seeding the user entity.
use chrono::Utc;
use fake::faker::internet::raw::SafeEmail;

use crate::entities::user::Model;
use crate::entities::user::{self};

use super::{create::CreateUser, GraphQLUser};
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::Fake;

pub fn get_mock_create_user_input() -> CreateUser {
    CreateUser {
        first_name: FirstName(EN).fake(),
        last_name: LastName(EN).fake(),
        email: SafeEmail(EN).fake(),
    }
}

pub fn get_mock_user_model(input: CreateUser) -> user::Model {
    user::Model {
        first_name: input.first_name,
        last_name: input.last_name,
        email: input.email,
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    }
}

pub fn get_mock_graphql_user(model: Model) -> GraphQLUser {
    GraphQLUser {
        created_at: model.created_at.timestamp(),
        updated_at: model.created_at.timestamp(),
        first_name: model.first_name,
        last_name: model.last_name,
        email: model.email,
    }
}
