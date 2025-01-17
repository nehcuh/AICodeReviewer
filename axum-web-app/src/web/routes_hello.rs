use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handle_hello2))
}

#[derive(Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handle_hello2(Path(params): Path<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handle_hello2", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>"))
}
