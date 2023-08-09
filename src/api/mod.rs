use axum::{
    routing::{post},
    Router,
};
use std::net::SocketAddr;

mod handlers;
pub async fn init() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/api/orders", post(handlers::orders::create_order));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
