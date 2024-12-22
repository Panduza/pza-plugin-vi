use async_trait::async_trait;
use panduza_platform_core::{std::class::acq_si::SiDataReader, Error};
use rand::Rng;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
///
/// Simple echo evaluation
///
pub struct RandomSiReader {}

impl RandomSiReader {
    pub fn into_arc_mutex(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }
}

#[async_trait]
impl SiDataReader for RandomSiReader {
    async fn read_data(&mut self, _channel: usize) -> Result<f64, Error> {
        let mut rng = rand::thread_rng();
        let random_value: f64 = rng.gen_range(0.0..0xffff as f64);
        Ok(random_value)
    }
}
