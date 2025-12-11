mod config;
mod models;
mod producer;
mod helper;
mod state;
mod constant;
use crate::config::{Config};
use crate::models::LogEntry;

use crate::helper::{formatted_timestamp, get_hostname, route_topic};
use crate::state::AppState;
use std::sync::Arc;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() ->anyhow::Result<()> {
    let cfg = Config::from_env_or_default();

    // let producer = Arc::new(KafkaProducer::new(cfg.clone())?);

    let state= AppState::new(&cfg)?;
    let mut count= 0;

    for _ in 0..10 {
        let level = match count % 3 {
            0 => "INFO",
            1 => "WARN",
            _ => "ERROR",
        }
        .to_string();

        let timestamp = formatted_timestamp();

        let entry = LogEntry {
            level: level.clone(),
            message: format!("This is log message number {}", count),
            hostname:get_hostname(),
            timestamp,
        };

        // decide topic based on routing
        let topic = route_topic(count);


        let producer_clone = Arc::clone(&state.producer);
        let entry_clone = entry.clone();
        let topic_clone = topic.clone();

        tokio::spawn(async move {
            match producer_clone.send(&topic_clone, entry_clone,&cfg.send_timeout).await {
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
