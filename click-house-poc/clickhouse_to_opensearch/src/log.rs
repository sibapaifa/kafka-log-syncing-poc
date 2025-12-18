use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Row, Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    #[serde(with = "clickhouse::serde::chrono::datetime64::nanos")]
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct OpenSearchLog<'a> {
    pub timestamp: &'a DateTime<Utc>,
    pub level: &'a str,
    pub message: &'a str,
}
impl<'a> OpenSearchLog<'a> {
    pub fn new(timestamp: &'a DateTime<Utc>, level: &'a str, message:&'a str) -> Self {
        Self {
            timestamp,
            level,
            message,
        }
    }
}
