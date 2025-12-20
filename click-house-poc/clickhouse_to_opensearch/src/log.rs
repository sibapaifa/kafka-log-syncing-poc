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

#[derive(Row, Debug, Serialize, Deserialize, Clone)]
pub struct InfoLog {
    #[serde(with = "clickhouse::serde::chrono::datetime64::nanos")]
    timestamp: DateTime<Utc>,
    information: String,
    action: String,
}

#[derive(Row, Clone, Debug, Serialize, Deserialize)]
pub struct WarnLog {
    #[serde(with = "clickhouse::serde::chrono::datetime64::nanos")]
    timestamp: DateTime<Utc>,
    ip: String,
    path: String,
    latency_ms: u64,
}
#[derive(Serialize, Debug)]
pub struct OpenSearchLog<'a> {
    pub timestamp: &'a DateTime<Utc>,
    pub level: &'a str,
    pub message: &'a str,
}
impl<'a> OpenSearchLog<'a> {
    pub fn new(timestamp: &'a DateTime<Utc>, level: &'a str, message: &'a str) -> Self {
        Self {
            timestamp,
            level,
            message,
        }
    }
}

pub trait GetTimestamp {
    fn get_timestamp(&self) -> DateTime<Utc>;
}

impl GetTimestamp for LogEntry {
    fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
impl GetTimestamp for InfoLog {
    fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
impl GetTimestamp for WarnLog {
    fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
