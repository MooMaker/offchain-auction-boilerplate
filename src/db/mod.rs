use redis::{Commands, RedisError};
use crate::config::Config;
use crate::models::RFQ;

pub struct DB {
    connection: redis::Connection,
}
pub const TABLE_RFQS: &str = "rfqs";
pub const SEPARATOR: &str = ":";

impl DB {
    pub fn new(config: &Config) -> Self {
        let redis_conn_url = format!("redis://{}", config.redis_hostname);

        let connection = redis::Client::open(redis_conn_url)
            .expect("Failed to open Redis connection")
            .get_connection()
            .expect("Failed to connect to Redis");

        Self {
            connection,
        }
    }

    pub fn create_rqf(&mut self, rfq: &RFQ) -> Result<(), RedisError> {
        let id = &rfq.id;

        let value = serde_json::to_string(rfq).unwrap();

        let key = format!("{}{}{}", TABLE_RFQS, SEPARATOR, id);

        let _:() = self.connection.set(&key, value)?;
        let _:() = self.connection.expire(key, rfq.time_limit as usize)?;
        Ok(())
    }

    pub fn get_rfq(&mut self, id: &str) -> Result<Option<RFQ>, RedisError> {
        let key = format!("{}{}{}", TABLE_RFQS, SEPARATOR, id);
        let value: Option<String> = self.connection.get(&key)?;

        Ok(value
            .map(|v| serde_json::from_str(&v).unwrap())
        )
    }
}
