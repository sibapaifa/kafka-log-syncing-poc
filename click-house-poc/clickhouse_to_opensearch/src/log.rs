use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Row, Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    #[serde(with = "clickhouse::serde::chrono::datetime64::nanos")]
    pub timestamp: DateTime<Utc>,
    level: String,
    pub message: String,
}