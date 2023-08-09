mod config;
mod api;
mod models;

#[tokio::main]
async fn main() {
    let config = config::init();

    api::init(&config).await;
}

