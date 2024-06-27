use tokio::net::TcpListener;

use axum::Router;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .merge(web::routes_hello::routes())
        .fallback_service(web::routes_static::routes());

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
