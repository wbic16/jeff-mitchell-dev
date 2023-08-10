// src/main.rs

// dependencies
use axum::Router;
use shuttle_axum::ShuttleAxum;
use std::path::PathBuf;
use tower_http::services::ServeDir;

// main function, annotated with the Shuttle runtime
// as well as Shuttle Static Folder (for static file hosting)
// and Shuttle Turso (for an edge SQlite database)
#[shuttle_runtime::main]
async fn main(
    #[shuttle_static_folder::StaticFolder(folder = "dist")] dist_folder: PathBuf,
) -> ShuttleAxum {
    let router = Router::new().nest_service("/", ServeDir::new(dist_folder));

    Ok(router.into())
}
