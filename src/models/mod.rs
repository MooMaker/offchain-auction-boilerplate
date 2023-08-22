use serde::{Serialize, Deserialize};

pub type TimeLimit = u16;

#[derive(Debug, Serialize, Deserialize)]
pub struct RFQ {
    pub id: String,
    pub sell_token: String,
    pub buy_token: String,
    pub sell_amount: String,
    pub buy_amount: String,
    pub time_limit: TimeLimit
}
