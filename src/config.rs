use dotenvy::dotenv;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_bind_address: String,
    pub redis_hostname: String,
}

pub fn init() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => return config,
        Err(error) => panic!("{:#?}", error)
    }
}
