mod config;
mod constant;
mod helper;
mod models;
mod producer;
mod state;
use std::time::Instant;

use crate::config::Config;
use crate::models::{ErrorLog, InfoLog, WarnLog};

use crate::helper::{formatted_timestamp, get_hostname};
use crate::state::AppState;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Config::from_env_or_default();

    let state = AppState::new(&cfg)?;
    println!("Application started.");
    let num_messages_per_type = 10; // Send a large number of messages
    let total_messages = num_messages_per_type * 3; // Info, Error, Warn

    let start_time = Instant::now();
    let mut sent_count = 0;

    for i in 0..num_messages_per_type {
        let warn_log_1 = WarnLog::new(
            "WARN".to_string(),
            format!("Cache not found, using default. Warn #{}", i),
            get_hostname(),
            formatted_timestamp(),
            "System Failure".to_string(),
        );
        
        state.producer.send(warn_log_1, &cfg.send_timeout).await?;
        sent_count += 1;
        let warn_log_2 = WarnLog::new(
            "WARN".to_string(),
            format!("Cache not found, using default. Warn #{}", i),
            get_hostname(),
            formatted_timestamp(),
            "System Failure".to_string(),
        );

        state.producer.send(warn_log_2, &cfg.send_timeout).await?;
        sent_count += 1;

        let warn_log = WarnLog::new(
            "WARN".to_string(),
            format!("Cache not found, using default. Warn #{}", i),
            get_hostname(),
            formatted_timestamp(),
            "System Failure".to_string(),
        );
        state.producer.send(warn_log, &cfg.send_timeout).await?;
        sent_count += 1;

        if sent_count % 1000 == 0 {
            println!("Sent {} messages...", sent_count);
        }
    }

    // Allow some time for final batches to be sent and delivered reports to arrive
    // This is important because `linger.ms` means messages aren't sent immediately.
    sleep(Duration::from_secs(5)).await;

    let duration = start_time.elapsed();
    println!(
        "\nFinished sending {} messages in {:?}. Average throughput: {:.2} msg/s",
        sent_count,
        duration,
        sent_count as f64 / duration.as_secs_f64()
    );

    // Give Kafka Connect some time to process if you're observing it simultaneously
    println!("Producer finished. Waiting for Kafka Connect to process messages...");
    sleep(Duration::from_secs(10)).await;

    println!("Application finished.");
    Ok(())
}
