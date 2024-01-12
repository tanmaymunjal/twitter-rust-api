use axum::{
    routing::{get, post},
    Router,
};
pub mod endpoints;
pub mod models;
mod postgres_connect;
pub mod schema;
pub mod utils;
use std::sync::Arc;

pub struct AppState {
    pub db_pool: postgres_connect::DbPool,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let connection = postgres_connect::establish_connection();

    let state = Arc::new(AppState {
        db_pool: connection,
    });

    let app = Router::new()
        .route("/user/:user_email", get(endpoints::get_user::get_user))
        .route("/user", post(endpoints::create_user::create_user))
        .route("/post", post(endpoints::create_post::create_post))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
