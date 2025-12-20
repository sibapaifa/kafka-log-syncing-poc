use chrono::{DateTime, Utc};
use clickhouse::{Client, Row};
use serde::{Deserialize, Serialize};

#[derive(Row, Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    #[serde(serialize_with = "clickhouse::serde::chrono::datetime64::nanos::serialize")]
    timestamp: DateTime<Utc>,
    level: String,
    message: String,
}

#[derive(Row, Debug, Serialize, Deserialize, Clone)]
pub struct InfoLog {
    #[serde(serialize_with = "clickhouse::serde::chrono::datetime64::nanos::serialize")]
    timestamp: DateTime<Utc>,
    information: String,
    action: String,
}

#[derive(Row, Clone, Debug, Serialize, Deserialize)]
pub struct WarnLog {
    timestamp: DateTime<Utc>,
    ip: String,
    path: String,
    latency_ms: u64,
}

async fn insert_into_clickhouse<T>(client: &Client, table: &str, log: &T) -> anyhow::Result<()>
where
    T: Row + Serialize + Send + Sync,
    for<'a> T: Row<Value<'a> = T>,
{
    let mut inserter = client.insert::<T>(table).await?;
    inserter.write(log).await?;
    inserter.end().await?;
    Ok(())
}

#[derive(Deserialize)]
pub enum Log {
    Info(InfoLog),
    Warn(WarnLog),
    Entry(LogEntry),
}

impl Log {
    // This looks like the trait, but it's just a method on the Enum
    pub async fn handle(&self, client: &clickhouse::Client) -> anyhow::Result<()> {
        match self {
            Log::Info(inner) => inner.handle(client).await,
            Log::Warn(inner) => inner.handle(client).await,
            Log::Entry(inner) => inner.handle(client).await,
        }
    }
}
#[async_trait::async_trait]
pub trait LogHandler: Send + Sync {
    async fn handle(&self, client: &Client) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl LogHandler for InfoLog {
    async fn handle(&self, client: &Client) -> anyhow::Result<()> {
        insert_into_clickhouse(client, "info_logs", self).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl LogHandler for WarnLog {
    async fn handle(&self, client: &Client) -> anyhow::Result<()> {
        insert_into_clickhouse(client, "warn_logs", self).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl LogHandler for LogEntry {
    async fn handle(&self, client: &Client) -> anyhow::Result<()> {
        insert_into_clickhouse(client, "logs", self).await?;
        Ok(())
    }
}
