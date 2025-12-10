mod config;
mod models;
mod producer;
mod helper;

use crate::config::AppConfig;
use crate::models::LogEntry;
use crate::producer::KafkaProducer;
use crate::helper::route_topic;
use hostname::get;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = AppConfig::from_env_or_default();

    let producer = Arc::new(KafkaProducer::new(cfg.clone())?);

    let hostname = get()
        .unwrap_or_default()
        .into_string()
        .unwrap_or_else(|_| "unknown".into());

    let mut count= 0;

    for _ in 0..10 {
        let level = match count % 3 {
            0 => "INFO",
            1 => "WARN",
            _ => "ERROR",
        }
        .to_string();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let entry = LogEntry {
            level: level.clone(),
            message: format!("This is log message number {}", count),
            hostname: hostname.clone(),
            timestamp,
        };

        // decide topic based on routing
        let topic = route_topic(count);


        let producer_clone = Arc::clone(&producer);
        let entry_clone = entry.clone();
        let topic_clone = topic.clone();

        tokio::spawn(async move {
            match producer_clone.send(&topic_clone, entry_clone).await {
                Ok(_) => println!("Delivered to topic {}", topic_clone),
                Err(e) => eprintln!("Failed to deliver to {}: {}", topic_clone, e),
            }
        });

        count += 1;
        sleep(Duration::from_secs(1)).await;
    }

    sleep(Duration::from_secs(2)).await;

    Ok(())
}
