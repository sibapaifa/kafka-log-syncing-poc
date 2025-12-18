use chrono::{DateTime, Utc};
use rand::Rng;
use rand::random;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::net::UdpSocket;
#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    timestamp: DateTime<Utc>,
    level: String,
    message: String,
    service: String,
    request_id: String,
    user_id: u32,
    duration_ms: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_addr = "127.0.0.1:9000"; // Vector will listen on this address
    let socket = UdpSocket::bind("0.0.0.0:0").await?; // Bind to an ephemeral port
    println!("Log producer started. Sending logs to {}", target_addr);

    let mut rng = rand::rng();
    let service_names = vec![
        "auth-service",
        "payment-service",
        "user-service",
        "product-service",
    ];
    let log_levels = vec!["INFO", "WARN", "ERROR", "DEBUG"];
    let messages = vec![
        "Request received successfully.",
        "Database query executed.",
        "User login failed.",
        "External API call timed out.",
        "Data processed.",
        "Configuration reloaded.",
        "Cache hit.",
        "Cache miss.",
        "Invalid input provided.",
        "Transaction committed.",
    ];

    for i in 1..5 {
        let service = service_names[rng.random_range(0..service_names.len())].to_string();
        let level = log_levels[rng.random_range(0..log_levels.len())].to_string();
        let message = messages[rng.random_range(0..messages.len())].to_string();

        let log_entry = LogEntry {
            timestamp: Utc::now(),
            level,
            message,
            service,
            request_id: format!("{:x}", random::<u64>()),
            user_id: rng.random_range(1000..=9999),
            duration_ms: rng.random_range(10..=2000),
        };
        let json_log = serde_json::to_string(&log_entry)?;

        println!(
            "Sending single log record ({} bytes): {}",
            json_log.len(),
            json_log
        );
        socket.send_to(json_log.as_bytes(), target_addr).await?;

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}
