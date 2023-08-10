use std::sync::{Arc, RwLock};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::{State},
};
use redis::RedisError;
use crate::models::Order;
use crate::state::DB;

type CreateOrder = Order;
pub async fn create_order(State(state): State<Arc<RwLock<DB>>>, Json(payload): Json<CreateOrder>) -> impl IntoResponse
{
    let order = payload as Order;

    let result = state.write().unwrap().create_rqf();

    match result {
        Ok(_) => (StatusCode::CREATED, Json(order)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to place order: {}", e.to_string()
        )).into_response()
    }
}
