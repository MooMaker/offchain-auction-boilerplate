use axum::{
    routing::{post},
    Router,
};
use std::net::SocketAddr;
use crate::config::Config;

mod handlers;
pub async fn init(config: &Config) {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/api/orders", post(handlers::orders::create_order));

    let addr: SocketAddr = config.api_bind_address.parse().unwrap();
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
