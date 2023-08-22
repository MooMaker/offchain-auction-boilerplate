mod config;
mod db;
pub mod api;
mod models;

use db::DB;

#[tokio::main]
async fn main() {
    let config = config::init();

    let db = DB::new(&config).await;

    api::init(&config, db).await;
}

