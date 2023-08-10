use std::fmt::{Debug, Formatter, Error};
use redis::{Commands, RedisError};
use crate::config::Config;

pub struct DB {
    connection: redis::Connection,
}

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

    pub fn create_rqf(&mut self) -> Result<(), RedisError> {
        let _:() = self.connection.set("foo", "bar")?;
        let bar: String = self.connection.get("foo")?;
        Ok(())
    }
}
