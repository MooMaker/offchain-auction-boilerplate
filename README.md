# POC and a boilerplate for off-chain auction service
Service implements a simple HTTP API for placing trade orders.

Once the order is placed, it is stored in Redis with specified TTL and broadcasted to a list of subscribed web soecket connections. 

TBD:
- Makers response processing
- Simple integration tests


## Tech stack
- Language: Rust
- Web Framework: [Axum](https://github.com/tokio-rs/axum)
- DB: [Redis](https://redis.io/)

## How to run 
- Install and run [Redis](https://redis.io/docs/getting-started/) server
- `$ cp .env.example .env`
- `$ cargo run`
- Open `test-client/index.html` in your browser to establish a WS connection with the service
- Send post request with an order
  
  `$ curl -X POST -H "Content-Type: application/json" -d '{"sell_token": "0x752efadc0a7e05ad1bcccda22c141d01a75ef1e4","buy_token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2","sell_amount": "296635689186984646942","buy_amount": "408457178963175500","time_limit": 10}' http://localhost:3000/api/orders`
- Check browser console for message from the service like `Message from server: RFQ <id> created` 

## Running tests
- Install and run [Redis](https://redis.io/docs/getting-started/) server
- `$ cp .env.example .env`
- `$ cargo test`
