use std::sync::{Arc, RwLock};
use axum::body::{Body};
use axum::http;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};

use tower::Service;

use offchain_auction::api::app;
use offchain_auction::config::Config;
use offchain_auction::db::DB;
use offchain_auction::models::RFQ;

#[tokio::test]
async fn it_creates_rfq_when_order_is_placed() {
    let config = Config::default();

    let db = DB::new(&config).await;

    let mut app = app(db);

    let create_order_response = app
        .call(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/orders")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(json!({
                    "sell_token": "0x752efadc0a7e05ad1bcccda22c141d01a75ef1e4",
                    "buy_token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                    "sell_amount": "296635689186984646942",
                    "buy_amount": "408457178963175500",
                    "time_limit": 10
                }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_order_response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(create_order_response.into_body()).await.unwrap();
    let rfq: RFQ = serde_json::from_slice(&body).unwrap();

    let get_order_response = app
        .call(
            Request::builder()
                .uri(format!("/api/orders/{}", rfq.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(get_order_response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body, json!({
        "id": rfq.id,
        "sell_token": "0x752efadc0a7e05ad1bcccda22c141d01a75ef1e4",
        "buy_token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "sell_amount": "296635689186984646942",
        "buy_amount": "408457178963175500",
        "time_limit": 10
    }));
}

