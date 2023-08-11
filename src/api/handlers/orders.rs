use std::sync::{Arc, RwLock};
use serde::{Deserialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::{State},
};
use crate::models::{RFQ, TimeLimit};
use crate::state::DB;

#[derive(Deserialize)]
pub struct Order {
    pub sell_token: String,
    pub buy_token: String,
    pub sell_amount: String,
    pub buy_amount: String,
    pub time_limit: TimeLimit
}

pub async fn place_order(State(state): State<Arc<RwLock<DB>>>, Json(payload): Json<Order>) -> impl IntoResponse
{
    // Generate RFQ id
    let rfq_id = uuid::Uuid::new_v4();

    // Create RFQ
    let rfq = RFQ {
        id: rfq_id.to_string(),
        sell_token: payload.sell_token,
        buy_token: payload.buy_token,
        sell_amount: payload.sell_amount,
        buy_amount: payload.buy_amount,
        time_limit: payload.time_limit
    };

    let result = state.write().unwrap().create_rqf(&rfq);

    match result {
        Ok(_) => (StatusCode::CREATED, Json(rfq)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to place order: {}", e.to_string()
        )).into_response()
    }
}
