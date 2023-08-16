use std::sync::{Arc};
use serde::{Deserialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::{State},
};
use axum::extract::Path;
use axum::extract::ws::Message;
use futures::SinkExt;
use crate::api::Context;
use crate::models::{RFQ, TimeLimit};

#[derive(Deserialize)]
pub struct Order {
    pub sell_token: String,
    pub buy_token: String,
    pub sell_amount: String,
    pub buy_amount: String,
    pub time_limit: TimeLimit
}

pub async fn get(State(state): State<Arc<Context>>, Path(id): Path<String>) -> impl IntoResponse {
    let result = state.db.write().unwrap().get_rfq(id.as_str()).unwrap();

    println!("RFQ: {:?}", result);

    result.map_or_else(|| (StatusCode::NOT_FOUND, "RFQ not found".to_string()).into_response(),
        |rfq| (StatusCode::OK, Json(rfq)).into_response()
    )
}

pub async fn create(State(state): State<Arc<Context>>, Json(payload): Json<Order>) -> impl IntoResponse
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

    // Write RFQ to DB
    let result = state.db.write().unwrap().create_rqf(&rfq);
    if let Err(e) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to place order: {}", e.to_string())
        ).into_response();
    }

    // Notify makers over websocket
    let mut makers = state.makers.lock().await;
    for maker in makers.values_mut() {
        println!("Sending RFQ {} to maker", rfq.id);
        let _ = maker.send(Message::Text(format!("RFQ {} created", rfq.id))).await;
    }

    // Return RFQ
    (StatusCode::CREATED, Json(rfq)).into_response()
}
