use async_trait::async_trait;
use panduza_platform_core::{Container, DriverOperations, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Default)]
///
///
pub struct Device {}

impl Device {
    /// Constructor
    ///
    pub fn new() -> Self {
        Device {}
    }
}

#[async_trait]
impl DriverOperations for Device {
    ///
    ///
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        let samples = instance
            .create_attribute("samples")
            .with_ro()
            .finish_as_sample()
            .await?;

        let number_of_point = 6000;
        let step = 0.05;

        let mut data = Vec::new();
        for i in 0..number_of_point {
            data.push(f32::sin(i as f32 * step));
        }

        samples.set(data).await?;

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
