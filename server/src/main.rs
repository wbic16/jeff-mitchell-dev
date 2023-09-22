// src/main.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use shuttle_axum::ShuttleAxum;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::info;

// health_check endpoint
// serves a 200 OK response with no body
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// main function, annotated with the Shuttle runtime
// as well as Shuttle Static Folder (for static file hosting)
// and Shuttle Turso (for an edge SQlite database)
#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .nest_service("/", ServeDir::new(PathBuf::from("/dist")));

    info!("Server listening on port 8000...");
    Ok(router.into())
}
