use axum::{routing::get, Router};
pub mod endpoints;
pub mod models;
mod postgres_connect;
pub mod schema;
use diesel::prelude::*;

struct AppState {
    pub db: PgConnection,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let connection = postgres_connect::establish_connection();

    let state = AppState { db: connection };

    let app = Router::new().route(
        "/user/:user_email",
        get(endpoints::get_user::get_user),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
