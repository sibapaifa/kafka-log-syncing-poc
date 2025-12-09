use hostname::get;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    level: String,
    message: String,
    hostname: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    let broker = "localhost:9092"; // Your Kafka broker address
    let topic = "log_topic"; // Your Kafka topic name

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let producer = Arc::new(producer);
    let hostname = get().unwrap_or_default().into_string().unwrap_or_default();

    println!("Starting Kafka producer to topic: {}", topic);

    let mut log_count = 0;
    loop {
        let level = match log_count % 3 {
            0 => "INFO",
            1 => "WARN",
            _ => "ERROR",
        };

        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let log_entry = LogEntry {
            level: level.to_string(),
            message: format!("This is log message number {}", log_count),
            hostname: hostname.clone(),
            timestamp: current_timestamp,
        };
        let log_json =
            serde_json::to_string(&log_entry).expect("Failed to serialize log entry to JSON");

        println!("Sending log: {}", log_json);

        let producer_clone = Arc::clone(&producer);
        // Clone log_json and hostname for the async task.
        // This ensures the async task owns its copy of the data.
        let log_json_for_task = log_json.clone();
        let hostname_for_task = hostname.clone();

        tokio::spawn(async move {
            let record = FutureRecord::to(topic)
                .payload(&log_json_for_task) // Now referencing data owned by the task
                .key(&hostname_for_task); // Now referencing data owned by the task

            match producer_clone.send(record, Duration::from_secs(0)).await {
                Ok(delivery) => println!("Delivered: {:?}", delivery),
                Err((e, _)) => eprintln!("Failed to deliver: {}", e),
            }
        });

        log_count += 1;
        sleep(Duration::from_secs(1)).await; // Send a log every second
    }
}
