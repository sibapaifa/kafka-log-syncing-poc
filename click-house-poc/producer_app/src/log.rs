use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    timestamp: DateTime<Utc>,
    level: String,
    message: String,
}

impl LogEntry {
    pub fn new(timestamp: DateTime<Utc>, level: String, message: String) -> Self {
        Self {
            timestamp,
            level,
            message,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoLog {
    timestamp: DateTime<Utc>,
    information: String,
    action: String,
}

impl InfoLog {
    pub fn new(timestamp: DateTime<Utc>, information: String, action: String) -> Self {
        Self {
            timestamp,
            information,
            action,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarnLog {
    timestamp: DateTime<Utc>,
    ip: String,
    path: String,
    latency_ms: u64,
}
impl WarnLog {
    pub fn new(timestamp: DateTime<Utc>, ip: String, path: String, latency_ms: u64) -> Self {
        Self {
            timestamp,
            ip,
            path,
            latency_ms,
        }
    }
}

#[derive(Serialize)]
pub enum Log {
    Info(InfoLog),
    Warn(WarnLog),
    Entry(LogEntry),
}
