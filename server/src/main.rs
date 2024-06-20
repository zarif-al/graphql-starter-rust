use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

mod graphql;

use graphql::{mutation::MutationRoot, query::QueryRoot};

use db::{env::get_env, get_db_connection};

/// GraphQL Playground
async fn graphiql() -> impl IntoResponse {
    let env = get_env();

    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .title(env.app_name.as_str())
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    // Load env from .env file
    dotenv().ok();

    // Get env struct
    let env = get_env();

    // Setup logger
    // Update the `with_env_filter()` to get more or less logs
    tracing_subscriber::fmt().with_env_filter("server").init();

    // Get db connection
    let db = get_db_connection().await;

    match db {
        Ok(db) => {
            // Build graphql schema
            let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
                // Add db to graphl context
                .data(db)
                .finish();

            // Build server
            let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

            // Serve
            info!(
                "Application Name: {}. Listening on port {}",
                env.app_name, env.port
            );

            let addr = SocketAddr::from(([0, 0, 0, 0], env.port));
            let tcp_listener = TcpListener::bind(addr).await;

            match tcp_listener {
                Ok(listener) => {
                    axum::serve(listener, app)
                        .await
                        .expect("App failed to startup!");
                }
                Err(err) => {
                    panic!("{}", err.to_string())
                }
            }
        }
        Err(err) => {
            panic!("{}", err.to_string());
        }
    }
}
