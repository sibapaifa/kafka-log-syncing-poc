use std::{fmt::Debug, fs};

use chrono::Utc;
use clickhouse::Row;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    Watermark,
    constant::{
        MAX_DOCS_PER_BATCH, MAX_PAYLOAD_SIZE_BYTES, OPENSEARCH_PASSWORD, OPENSEARCH_URL,
        OPENSEARCH_USER,
    },
    log::{GetTimestamp, ToOpenSearch},
};

pub fn read_watermark(table_name: &str) -> Watermark {
    let path = format!("watermark_{}.json", table_name);
    std::fs::read_to_string(path)
        .and_then(|content| serde_json::from_str(&content).map_err(|e| e.into()))
        .unwrap_or_else(|_| Watermark {
            last_processed_timestamp: Utc::now() - chrono::Duration::days(1),
        })
}

pub fn write_watermark(table_name: &str, watermark: &Watermark) -> Result<(), std::io::Error> {
    let path = format!("watermark_{}.json", table_name);
    let data = serde_json::to_string_pretty(watermark)?;
    fs::write(path, data)?;
    Ok(())
}

pub async fn send_to_opensearch<T: Serialize>(
    index_name: &str,
    all_new_logs: &[T],
    http_client: &Client,
) -> anyhow::Result<()> {
    if all_new_logs.is_empty() {
        return Ok(());
    }

    // --- Batch Processing Logic ---
    for (batch_num, log_batch) in all_new_logs.chunks(MAX_DOCS_PER_BATCH).enumerate() {
        let mut sub_batch_start_index = 0;

        while sub_batch_start_index < log_batch.len() {
            let mut current_payload_size = 0;
            let mut sub_batch_end_index = sub_batch_start_index;
            let mut body_str = String::new();

            // Create a sub-batch based on actual serialized size
            while sub_batch_end_index < log_batch.len() {
                let log = &log_batch[sub_batch_end_index];

                // 1. Serialize the log and the action header to get actual size
                let action = serde_json::json!({ "index": { "_index": index_name } });
                let action_str = serde_json::to_string(&action)?;
                let log_str = serde_json::to_string(&log)?;

                // +2 for the two newlines (\n)
                let total_line_size = action_str.len() + log_str.len() + 2;

                if current_payload_size + total_line_size > MAX_PAYLOAD_SIZE_BYTES
                    && current_payload_size > 0
                {
                    break;
                }

                // 2. Build the NDJSON body string
                body_str.push_str(&action_str);
                body_str.push('\n');
                body_str.push_str(&log_str);
                body_str.push('\n');

                current_payload_size += total_line_size;
                sub_batch_end_index += 1;
            }

            println!(
                "Table [{}]: Sending batch {} containing {} logs ({} bytes).",
                index_name,
                batch_num + 1,
                (sub_batch_end_index - sub_batch_start_index),
                current_payload_size
            );

            // 3. Send to OpenSearch
            let response = http_client
                .post(OPENSEARCH_URL)
                .basic_auth(OPENSEARCH_USER, Some(OPENSEARCH_PASSWORD))
                .header("Content-Type", "application/x-ndjson")
                .body(body_str)
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await?;
                return Err(anyhow::anyhow!(
                    "OpenSearch Error (Status {}): {}",
                    status,
                    error_text
                ));
            }

            sub_batch_start_index = sub_batch_end_index;
        }
    }

    Ok(())
}

pub async fn sync_table_to_opensearch<T:ToOpenSearch>(
    table_name: &str,
    clickhouse_client: &clickhouse::Client,
    http_client: &reqwest::Client,
) -> anyhow::Result<()>
where
    T: clickhouse::Row + Serialize + for<'b> Deserialize<'b> + Send + Sync + Debug + 'static,
    for<'a> T: Row<Value<'a> = T>,
    T: GetTimestamp,
{
    let watermark = read_watermark(table_name);
    let formatted_ts = watermark
        .last_processed_timestamp
        .format("%Y-%m-%d %H:%M:%S.%f")
        .to_string();

    // 1. Fetch from ClickHouse
    let query = format!(
        "SELECT * FROM {} WHERE timestamp > toDateTime64(?, 9, 'UTC') ORDER BY timestamp",
        table_name
    );
    let mut cursor = clickhouse_client
        .query(&query)
        .bind(formatted_ts)
        .fetch::<T>()?;

    let mut logs = Vec::new();
    while let Some(log) = cursor.next().await? {
        println!("##{:?}", log);
        logs.push(log);
    }

    if logs.is_empty() {
        println!("No logs found");
        return Ok(());
    }
    println!("--->{:?}", logs);
    let os_logs: Vec<serde_json::Value> = logs
        .iter()
        .map(|log| log.to_json_value())
        .collect::<anyhow::Result<Vec<_>>>()?;
    // 2. Send to OpenSearch (using your existing batching logic, made generic)
    send_to_opensearch(table_name, &os_logs, http_client).await?;

    // 3. Update Watermark   let os_logs: Vec<T> = logs.iter().map(|log| (&log).into()).collect();
    if let Some(last_log) = logs.last() {
        write_watermark(
            table_name,
            &Watermark {
                last_processed_timestamp: last_log.get_timestamp(),
            },
        )?;
    }

    Ok(())
}
