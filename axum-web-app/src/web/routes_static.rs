use axum::{routing::get_service, Router};

pub fn routes() -> Router {
    Router::new().route("/", get_service(tower_http::services::ServeDir::new("./")))
}
