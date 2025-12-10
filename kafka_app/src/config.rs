use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub brokers: String,
    pub default_topic: String,
    pub send_timeout: Duration,
}

impl AppConfig {
    pub fn from_env_or_default() -> Self {
        let brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".into());
        let default_topic = std::env::var("KAFKA_TOPIC").unwrap_or_else(|_| "rust-app-logs".into());
        let send_timeout = Duration::from_secs(5);

        Self {
            brokers,
            default_topic,
            send_timeout,
        }
    }
}
