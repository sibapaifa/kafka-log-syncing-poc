use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub hostname: String,
    pub timestamp: String,
}