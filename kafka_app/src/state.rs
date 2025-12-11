use std::sync::Arc;

use crate::{config::Config, producer::KafkaProducer};

#[derive(Clone)]
pub struct AppState {
    pub producer: Arc<KafkaProducer>,
}

impl AppState {
    pub fn new(cfg: &Config) -> anyhow::Result<Self> {
        let kafka_producer = KafkaProducer::new(cfg)?;

        Ok(Self {
            producer: Arc::new(kafka_producer)
        })
    }
}
