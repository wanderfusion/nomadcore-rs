mod routes;
mod middlewares;

use axum::{
    routing::get,
    Router, middleware,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(routes::health_check::health_check_handler))
        .layer(middleware::from_fn(middlewares::log::log_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
