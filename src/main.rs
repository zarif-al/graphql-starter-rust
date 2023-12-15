use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

mod general_response;
mod graphql;
mod health_check;
mod user;

use graphql::{mutation::MutationRoot, query::QueryRoot};

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Build graphql schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    info!("Application started!. Listening on port  4000");

    // setup server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    // serve it with hyper on designated port
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("App failed to startup!");
}
