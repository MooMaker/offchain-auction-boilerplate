use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub sell_token: String,
    pub buy_token: String,
    pub sell_amount: String,
    pub buy_amount: String,
}
