use std::net::{Ipv4Addr, SocketAddr};
use axum::body::{Body};
use axum::http;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use futures::SinkExt;
use futures::StreamExt;

use tower::Service;

use tokio_tungstenite::tungstenite;
use offchain_auction::api::app;
use offchain_auction::config::Config;
use offchain_auction::db::DB;
use offchain_auction::models::RFQ;

fn mock_order() -> serde_json::Value {
    json!({
        "sell_token": "0x752efadc0a7e05ad1bcccda22c141d01a75ef1e4",
        "buy_token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "sell_amount": "296635689186984646942",
        "buy_amount": "408457178963175500",
        "time_limit": 10
    })
}

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
                .body(Body::from(mock_order().to_string()))
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

    let mut expected_order = mock_order();
    expected_order.as_object_mut().unwrap().insert("id".to_string(), json!(rfq.id));

    assert_eq!(body, expected_order);
}

#[tokio::test]
async fn it_notifies_makers_about_rfq() {
    let config = Config::default();

    let db = DB::new(&config).await;

    let server = axum::Server::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))
        .serve(app(db).into_make_service_with_connect_info::<SocketAddr>());
    let addr = server.local_addr();
    tokio::spawn(server);


    // Simulate MM connection to a websocket server
    let (mut socket, _response) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/ws"))
            .await
            .unwrap();

    // Send Pong msg to server in response to ping
    match socket.next().await.unwrap().unwrap() {
        tungstenite::Message::Ping(_) => {
            socket.send(tungstenite::Message::Pong(vec![])).await.unwrap();
        },
        other => panic!("expected ping message from the server but got {other:?}"),
    };

    // Simulate trading client that places an order
    let client = hyper::Client::new();

    let create_order_response = client
        .request(
        Request::builder()
                .method(http::Method::POST)
                .uri(format!("http://{addr}/api/orders"))
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(mock_order().to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(create_order_response.into_body()).await.unwrap();
    let rfq: RFQ = serde_json::from_slice(&body).unwrap();

    // Expect to receive RFQ notification from the server
    let message = match socket.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(text) => {
            text
        },
        other => panic!("expected ping message from the server but got {other:?}"),
    };

    assert_eq!(message, format!("RFQ {} created", rfq.id));
}
