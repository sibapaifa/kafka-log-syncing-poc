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
