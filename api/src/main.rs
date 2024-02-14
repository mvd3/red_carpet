use axum::{
    routing::{get, post, patch},
    Router,
};
use tokio;
use std::sync::{Arc, Mutex};

mod handlers;
mod models;
mod database;

const ADDRESS: &str = "0.0.0.0:5000";

#[tokio::main]
async fn main() {
    let database = Arc::new(Mutex::new(database::Database::new()));
    let app = Router::new()
        .route("/check", get(handlers::check_handler))
        .route("/createRequest", post(handlers::create_request_handler))
        .route("/onboardingList", get(handlers::onboarding_list_handler))
        .route("/addUser/:id", patch(handlers::add_user_handler))
        .route("/userDetails", get(handlers::user_details_handler))
        .layer(axum::Extension(database));

    let listener = tokio::net::TcpListener::bind(ADDRESS).await.unwrap();
    println!("Listening on {}", ADDRESS);

    axum::serve(listener, app).await.unwrap();
}
