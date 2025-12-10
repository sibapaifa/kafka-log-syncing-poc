use crate::config::AppConfig;
use crate::models::LogEntry;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    inner: Arc<FutureProducer>,
    cfg: AppConfig,
}

impl KafkaProducer {
    pub fn new(cfg: AppConfig) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.brokers)
            .set(
                "message.timeout.ms",
                &cfg.send_timeout.as_millis().to_string(),
            )
            .create()?;

        Ok(Self {
            inner: Arc::new(producer),
            cfg,
        })
    }
    pub async fn send(
        &self,
        topic: &str,
        entry: LogEntry,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_string(&entry)?;
        let key = entry.hostname.clone();
        let record = FutureRecord::to(topic).payload(&payload).key(&key);

        let timeout = self.cfg.send_timeout;
        match self.inner.send(record, Duration::from(timeout)).await {
            Ok(_delivery) => Ok(()),
            Err((err, _owned_message)) => Err(Box::new(err)),
        }
    }
}
