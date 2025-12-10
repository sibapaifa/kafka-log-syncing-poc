use std::sync::Arc;

use crate::{config::AppConfig, producer::KafkaProducer};

#[derive(Clone)]
pub struct AppState {
    pub producer: Arc<KafkaProducer>,
}

impl AppState {
    pub fn new(cfg: AppConfig) -> anyhow::Result<Self> {
        let kafka_producer = KafkaProducer::new(cfg.clone())?;

        Ok(Self {
            producer: Arc::new(kafka_producer)
        })
    }
}
