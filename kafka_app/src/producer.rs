use crate::config::Config;
use crate::helper::get_hostname;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    pub inner: FutureProducer,
}

impl KafkaProducer {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &cfg.broker)
            .set(
                "message.timeout.ms",
                cfg.send_timeout.as_millis().to_string()
            )
            .create()?;

        Ok(Self { inner: producer })
    }
    pub async fn send<T: Serialize>(
        &self,
        topic: &str,
        entry: T,
        send_timeout: &Duration,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_string(&entry)?;
        let key = get_hostname();
        //Key is used for partioning , If we want a roound robin fashion we should not specify key !
        // let record = FutureRecord::<(),_>::to(topic).payload(&payload);
        //Key is used for partioning , If we want a roound robin fashion we should not specify key !
        let record = FutureRecord::to(topic).payload(&payload).key(&key);
        match self.inner.send(record, *send_timeout).await {
            Ok(_delivery) => Ok(()),
            Err((err, _owned_message)) => Err(Box::new(err)),
        }
    }
}
