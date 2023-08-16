use dotenvy::dotenv;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_bind_address: String,
    pub redis_hostname: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_bind_address: "localhost:3000".to_string(),
            redis_hostname: "localhost:6379".to_string(),
        }
    }
}

pub fn init() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => return config,
        Err(error) => panic!("{:#?}", error)
    }
}
