use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::{Message};
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

// This struct should match the LogEntry in your producer
#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    level: String,
    message: String,
    hostname: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    let broker = "localhost:9092";      // Your Kafka broker address
    let topic = "log_topic";          // The Kafka topic to consume from
    let group_id = "opensearch_consumer_group"; // Consumer group ID

    // OpenSearch configuration
    let opensearch_url = ""; // Replace with your OpenSearch URL
    let opensearch_user = ""; // Replace with your OpenSearch username
    let opensearch_password = ""; // Replace with your OpenSearch password
    let opensearch_index = "test-application-logs"; // OpenSearch index name

    // Base64 encode the credentials for Basic Auth
    let auth_string = format!("{}:{}", opensearch_user, opensearch_password);
    let encoded_auth = base64::encode(auth_string.as_bytes());
    let auth_header_value = format!("Basic {}", encoded_auth);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", broker)
        .set("enable.auto.commit", "false") // Manually commit offsets for reliability
        .set("auto.offset.reset", "earliest") // Start from the beginning if no offset is found
        .create()
        .expect("Consumer creation error");

    consumer.subscribe(&[topic])
        .expect("Can't subscribe to specified topic");

    println!("Starting Kafka consumer for topic: {} in group: {}", topic, group_id);

    let http_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // ONLY FOR DEVELOPMENT! In production, set up proper CA certificates.
        .build()
        .expect("Failed to create HTTP client");


    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let payload_str = match msg.payload_view::<str>() {
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        eprintln!("Error deserializing message payload: {:?}", e);
                        continue;
                    },
                    None => {
                        println!("Received message with no payload");
                        continue;
                    }
                };

                println!("Received message: {}", payload_str);

                match serde_json::from_str::<LogEntry>(payload_str) {
                    Ok(log_entry) => {
                        // Send to OpenSearch
                        let opensearch_doc = json!({
                            "level": log_entry.level,
                            "message": log_entry.message,
                            "hostname": log_entry.hostname,
                            "timestamp": log_entry.timestamp,
                            "@timestamp": chrono::Utc::now().to_rfc3339() // Add a standard timestamp for OpenSearch
                        });

                        let client_clone = http_client.clone();
                        let opensearch_url_clone = opensearch_url.to_string();
                        let opensearch_index_clone = opensearch_index.to_string();
                        let auth_header_value_clone = auth_header_value.clone();

                        tokio::spawn(async move {
                            let doc_url = format!("{}/{}/_doc", opensearch_url_clone, opensearch_index_clone);
                            match client_clone.post(&doc_url)
                                .header(AUTHORIZATION, &auth_header_value_clone)
                                .header(CONTENT_TYPE, "application/json")
                                .json(&opensearch_doc)
                                .send()
                                .await
                            {
                                Ok(response) => {
                                    if response.status().is_success() {
                                        println!("Successfully sent log to OpenSearch: {:?}", opensearch_doc);
                                    } else {
                                        eprintln!("Failed to send log to OpenSearch. Status: {}, Response: {:?}", response.status(), response.text().await);
                                    }
                                },
                                Err(e) => eprintln!("Error sending log to OpenSearch: {:?}", e),
                            }
                        });
                    },
                    Err(e) => {
                        eprintln!("Failed to parse JSON log: {} Error: {}", payload_str, e);
                    }
                }
                // Manually commit offset after successful processing
                if let Err(e) = consumer.commit_message(&msg, rdkafka::consumer::CommitMode::Async) {
                    eprintln!("Error committing offset: {}", e);
                }
            },
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}