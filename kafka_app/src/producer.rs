use crate::config::AppConfig;
use crate::models::LogEntry;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    pub inner: Arc<FutureProducer>,
}

impl KafkaProducer {
    pub fn new(cfg: AppConfig) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.broker)
            .set(
                "message.timeout.ms",
                &cfg.send_timeout.as_millis().to_string()
            )
            .create()?;

        Ok(Self {
            inner: Arc::new(producer),
        })
    }
    pub async fn send(
        &self,
        topic: &str,
        entry: LogEntry,
        send_timeout:&Duration
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_string(&entry)?;
        let key = entry.hostname.clone();
        let record = FutureRecord::to(topic).payload(&payload).key(&key);
        match self.inner.send(record, *send_timeout).await {
            Ok(_delivery) => Ok(()),
            Err((err, _owned_message)) => Err(Box::new(err)),
        }
    }
}
