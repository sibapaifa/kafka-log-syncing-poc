use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

use crate::constant::{
    CLICKHOUSE_PASSWORD, CLICKHOUSE_URL, CLICKHOUSE_USER, MAX_DOCS_PER_BATCH,
    MAX_PAYLOAD_SIZE_BYTES, OPENSEARCH_PASSWORD, OPENSEARCH_URL, OPENSEARCH_USER,
};
use crate::helper::{read_watermark, write_watermark};
use crate::log::{LogEntry, OpenSearchLog};
mod constant;
mod helper;
mod log;

#[derive(Serialize, Deserialize, Debug)]
struct Watermark {
    last_processed_timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Client Configurations ---
    let clickhouse_client = clickhouse::Client::default()
        .with_url(CLICKHOUSE_URL)
        .with_user(CLICKHOUSE_USER)
        .with_password(CLICKHOUSE_PASSWORD);

    let opensearch_url = OPENSEARCH_URL;
    let opensearch_user = OPENSEARCH_USER;
    let opensearch_pass = OPENSEARCH_PASSWORD;
    let http_client = Client::new();

    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        println!("Checking for new logs...");

        let last_processed_timestamp = read_watermark().unwrap_or_else(|_| Watermark {
            last_processed_timestamp: Utc::now() - chrono::Duration::days(1),
        });
        let formatted_ts = last_processed_timestamp
            .last_processed_timestamp
            .format("%Y-%m-%d %H:%M:%S.%f")
            .to_string();

        let mut cursor = clickhouse_client
            .query(
                "SELECT timestamp, level, message
         FROM logs
         WHERE timestamp > toDateTime64(?, 9, 'UTC')
         ORDER BY timestamp ASC",
            )
            .bind(formatted_ts)
            .fetch::<LogEntry>()?;

        let mut all_new_logs = Vec::new();
        while let Some(log) = cursor.next().await? {
            all_new_logs.push(log);
        }
        println!("{:?}", all_new_logs);
        if all_new_logs.is_empty() {
            println!("No new logs to send.");
            continue;
        }

        println!(
            "Found {} total new logs. Sending in batches...",
            all_new_logs.len()
        );

        // --- Batch Processing Logic ---
        // 'chunks' is an iterator that provides slices of our log vector.
        for (batch_num, log_batch) in all_new_logs.chunks(MAX_DOCS_PER_BATCH).enumerate() {
            // Further split the chunk if its estimated size is too large.
            // This is a more precise way to handle the size limit.
            let mut sub_batch_start_index = 0;
            while sub_batch_start_index < log_batch.len() {
                let mut current_payload_size = 0;
                let mut sub_batch_end_index = sub_batch_start_index;

                // Create a sub-batch based on estimated payload size
                while sub_batch_end_index < log_batch.len() {
                    let log = &log_batch[sub_batch_end_index];
                    // Estimate size: action JSON + log JSON + 2 newlines
                    let estimated_line_size = log.message.len() + 100; // Rough but safe estimation

                    if current_payload_size + estimated_line_size > MAX_PAYLOAD_SIZE_BYTES {
                        break; // This log would push the batch over the limit
                    }
                    current_payload_size += estimated_line_size;
                    sub_batch_end_index += 1;
                }

                // If the sub-batch is empty, it means a single log is too large.
                // We'll send it anyway to see the error, or you could add specific handling.
                if sub_batch_start_index == sub_batch_end_index {
                    sub_batch_end_index += 1;
                }

                let sub_batch = &log_batch[sub_batch_start_index..sub_batch_end_index];
                println!(
                    "Processing batch #{}: Sending {} logs ({} bytes estimated).",
                    batch_num + 1,
                    sub_batch.len(),
                    current_payload_size
                );

                // --- Construct and Send the Request for the current sub-batch ---
                let mut body_str = String::new();
                for log in sub_batch {
                    let opensearch_log = OpenSearchLog::new(&log.timestamp, &log.level, &log.message);
                    let action = serde_json::json!({ "index": {} });
                    let log_str = serde_json::to_string(&opensearch_log)?;
                    body_str.push_str(&serde_json::to_string(&action)?);
                    body_str.push('\n');
                    body_str.push_str(&log_str);
                    body_str.push('\n');
                }
                println!("{}", body_str);
                let response = http_client
                    .post(opensearch_url)
                    .basic_auth(opensearch_user, Some(opensearch_pass))
                    .header("Content-Type", "application/x-ndjson")
                    .body(body_str)
                    .send()
                    .await?;

                if !response.status().is_success() {
                    let status = response.status();
                    let response_body = response.text().await?;
                    eprintln!(
                        "Error sending batch #{}. Status: {}. Response: {}",
                        batch_num + 1,
                        status,
                        response_body
                    );
                    // If one batch fails, we stop and retry the whole set of logs on the next run.
                    // We achieve this by not updating the watermark.
                    return Err(format!("Failed to send batch #{}", batch_num + 1).into());
                }

                println!("Successfully sent batch #{}.", batch_num + 1);
                sub_batch_start_index = sub_batch_end_index; // Move to the start of the next sub-batch
            }
        }

        // --- Update Watermark Only After All Batches Succeed ---
        if let Some(last_log) = all_new_logs.last() {
            let new_watermark = Watermark {
                last_processed_timestamp: last_log.timestamp,
            };
            write_watermark(&new_watermark)?;
            println!(
                "All batches sent successfully. Updated watermark to: {}",
                new_watermark.last_processed_timestamp
            );
        }
    }
}
