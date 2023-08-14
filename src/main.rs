mod config;
mod db;
mod api;
mod models;

use std::sync::{Arc, RwLock};

use db::DB;

#[tokio::main]
async fn main() {
    let config = config::init();

    let db = Arc::new(RwLock::new(DB::new(&config)));

    api::init(&config, Arc::clone(&db)).await;
}

