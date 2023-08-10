use std::collections::HashMap;
use axum::{
    routing::{post},
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use axum::response::IntoResponse;
use crate::config::Config;
use crate::state::{DB};

mod handlers;


pub async fn init(config: &Config, state: Arc<RwLock<DB>>)
{
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/api/orders", post(handlers::orders::create_order))
        .with_state(state);

    let addr: SocketAddr = config.api_bind_address.parse().unwrap();
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
