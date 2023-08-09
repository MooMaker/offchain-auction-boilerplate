use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json
};
use crate::models::Order;

type CreateOrder = Order;
pub async fn create_order(Json(payload): Json<CreateOrder>) -> impl IntoResponse {
    // insert your application logic here
    let order = payload as Order;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(order))
}
