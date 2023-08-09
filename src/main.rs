mod api;
mod models;

#[tokio::main]
async fn main() {
    api::init().await;
}

