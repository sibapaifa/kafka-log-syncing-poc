use std::{env, time::Duration};

use dotenvy::dotenv;

use crate::constant::{BROKER, TIME_OUT};

#[derive(Clone)]
pub struct Config {
    pub broker: String,
    pub send_timeout: Duration,
}

impl Config {
    pub fn from_env_or_default() -> Self {
        dotenv().ok();

        let broker = env::var(BROKER).unwrap_or_else(|_| "localhost:9092".into());
        let send_timeout = Duration::from_secs(
            env::var(TIME_OUT)
                .ok() 
                .and_then(|s| s.parse::<u64>().ok()) 
                .unwrap_or(5),
        );

        Self {
            broker,
            send_timeout,
        }
    }
}
