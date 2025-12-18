use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use clickhouse::{Row};
#[derive(Row, Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    #[serde(serialize_with = "clickhouse::serde::chrono::datetime64::nanos::serialize")]
    timestamp: DateTime<Utc>,
    level: String,
    message: String,
}