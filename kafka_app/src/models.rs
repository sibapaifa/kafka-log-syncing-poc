use serde::{Deserialize, Serialize};

use crate::constant::{ERROR_TOPIC, INFO_TOPIC, WARN_TOPIC};

pub trait Loggable: Serialize + Send + Sync + 'static {
    fn topic(&self) -> &'static str;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfoLog {
    level: String,
    message: String,
    hostname: String,
    timestamp: String,
}

impl InfoLog {
    pub fn new(level: String, message: String, hostname: String, timestamp: String) -> Self {
        Self {
            level,
            message,
            hostname,
            timestamp,
        }
    }
}

impl Loggable for InfoLog {
    fn topic(&self) -> &'static str {
        INFO_TOPIC
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorLog {
    level: String,
    message: String,
    hostname: String,
    timestamp: String,
    error_code: u64,
}

impl ErrorLog {
    pub fn new(
        level: String,
        message: String,
        hostname: String,
        timestamp: String,
        error_code: u64,
    ) -> Self {
        Self {
            level,
            message,
            hostname,
            timestamp,
            error_code,
        }
    }
}

impl Loggable for ErrorLog {
    fn topic(&self) -> &'static str {
        ERROR_TOPIC
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct WarnLog {
    level: String,
    message: String,
    hostname: String,
    timestamp: String,
    reason: String,
}

impl WarnLog {
    pub fn new(
        level: String,
        message: String,
        hostname: String,
        timestamp: String,
        reason: String,
    ) -> Self {
        Self {
            level,
            message,
            hostname,
            timestamp,
            reason,
        }
    }
}

impl Loggable for WarnLog {
    fn topic(&self) -> &'static str {
        WARN_TOPIC
    }
}
