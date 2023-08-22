use std::collections::HashMap;
use axum::{routing::{post}, Router};
use std::net::SocketAddr;
use std::sync::{Arc};
use axum::extract::ws::{Message, WebSocket};
use axum::routing::get;
use futures::stream::SplitSink;
use tokio::sync::Mutex;
use crate::config::Config;
use crate::db::{DB};

mod handlers;

#[derive(Clone)]
pub struct Context {
    db: DB,
    makers: Arc<Mutex<HashMap<String, SplitSink<WebSocket, Message>>>>
}

pub async fn init(config: &Config, db: DB)
{
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = app(db);

    let addr: SocketAddr = config.api_bind_address.parse().unwrap();
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

pub fn app(db: DB) -> Router {
    let state = Context {
        db,
        makers: Arc::new(Mutex::new(HashMap::new()))
    };

    // build our application with a route
    Router::new()
        .route("/api/orders", post(handlers::orders::create))
        .route("/api/orders/:id",get(handlers::orders::get))
        .route("/ws", get(handlers::ws::ws_handler))
        .with_state(state)
}
