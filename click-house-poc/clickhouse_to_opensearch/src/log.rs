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
pub struct OpenSearchLogEntry<'a> {
    timestamp: &'a DateTime<Utc>,
    level: &'a str,
    message: &'a str,
}

impl<'a> From<&'a LogEntry> for OpenSearchLogEntry<'a> {
    fn from(log: &'a LogEntry) -> Self {
        Self {
            timestamp: &log.timestamp,
            level: &log.level,
            message: &log.message,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct OpenSearchInfoLog<'a> {
    timestamp: &'a DateTime<Utc>,
    information: &'a str,
    action: &'a str,
}

impl<'a> From<&'a InfoLog> for OpenSearchInfoLog<'a> {
    fn from(log: &'a InfoLog) -> Self {
        Self {
            timestamp: &log.timestamp,
            information: &log.information,
            action: &log.action,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct OpenSearchWarnLog<'a> {
    timestamp: &'a DateTime<Utc>,
    ip: &'a str,
    path: &'a str,
    latency_ms: &'a u64,
}

impl<'a> From<&'a WarnLog> for OpenSearchWarnLog<'a> {
    fn from(log: &'a WarnLog) -> Self {
        Self {
            timestamp: &log.timestamp,
            ip: &log.ip,
            path: &log.path,
            latency_ms: &log.latency_ms,
        }
    }
}

pub trait ToOpenSearch {
    fn to_json_value(&self) -> anyhow::Result<serde_json::Value>;
}
impl ToOpenSearch for LogEntry {
    fn to_json_value(&self) -> anyhow::Result<serde_json::Value> {
        // Convert to the OpenSearch-friendly struct, then to Value
        let val = serde_json::to_value(OpenSearchLogEntry::from(self))?;
        Ok(val)
    }
}

impl ToOpenSearch for InfoLog {
    fn to_json_value(&self) -> anyhow::Result<serde_json::Value> {
        let val = serde_json::to_value(OpenSearchInfoLog::from(self))?;
        Ok(val)
    }
}

impl ToOpenSearch for WarnLog {
    fn to_json_value(&self) -> anyhow::Result<serde_json::Value> {
        let val = serde_json::to_value(OpenSearchWarnLog::from(self))?;
        Ok(val)
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
