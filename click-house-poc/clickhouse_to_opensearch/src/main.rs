use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

use crate::constant::{CLICKHOUSE_PASSWORD, CLICKHOUSE_URL, CLICKHOUSE_USER};
use crate::helper::sync_table_to_opensearch;

use crate::log::{InfoLog, LogEntry, WarnLog};

mod constant;
mod helper;
mod log;

#[derive(Serialize, Deserialize, Debug)]
struct Watermark {
    last_processed_timestamp: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let clickhouse_client = clickhouse::Client::default()
        .with_url(CLICKHOUSE_URL)
        .with_user(CLICKHOUSE_USER)
        .with_password(CLICKHOUSE_PASSWORD);

    let http_client = Client::new();
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        println!("Checking for new logs...");

        let sync_entries =
            sync_table_to_opensearch::<LogEntry>("logs", &clickhouse_client, &http_client);
        let sync_info =
            sync_table_to_opensearch::<InfoLog>("info_logs", &clickhouse_client, &http_client);
        let sync_warn =
            sync_table_to_opensearch::<WarnLog>("warn_logs", &clickhouse_client, &http_client);

        let results = tokio::join!(sync_entries, sync_info, sync_warn);

        if let (Err(e1), Err(e2), Err(e3)) = results {
            eprintln!("Errors occurred during sync: {:?} {:?} {:?}", e1, e2, e3);
        }
    }
}
