mod config;
mod constant;
mod helper;
mod models;
mod producer;
mod state;
use crate::config::Config;
use crate::models::{ErrorLog, InfoLog, WarnLog};

use crate::helper::{formatted_timestamp, get_hostname};
use crate::state::AppState;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Config::from_env_or_default();

    let state = AppState::new(&cfg)?;
    println!("Application started.");
    for _ in 0..3 {
        let info_log = InfoLog::new(
            "INFO".to_string(),
            "Application initialized.".to_string(),
            get_hostname(),
            formatted_timestamp(),
        );
        state.producer.send(info_log, &cfg.send_timeout).await?;

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let error_log = ErrorLog::new(
            "ERROR".to_string(),
            "Failed to connect to database.".to_string(),
            get_hostname(),
            formatted_timestamp(),
            1001,
        );
        state.producer.send(error_log, &cfg.send_timeout).await?;

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        let warn_log = WarnLog::new(
            "WARN".to_string(),
            "Cache not found, using default.".to_string(),
            get_hostname(),
            formatted_timestamp(),
            "System Failure".to_string(),
        );
        state.producer.send(warn_log, &cfg.send_timeout).await?;
    }
    sleep(Duration::from_secs(2)).await;
    println!("Application finished.");
    Ok(())
}
