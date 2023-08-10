mod config;
mod state;
mod api;
mod models;

use std::sync::{Arc, RwLock};

use state::DB;

#[tokio::main]
async fn main() {
    let config = config::init();

    let state = Arc::new(RwLock::new(DB::new(&config)));

    api::init(&config, Arc::clone(&state)).await;
}

