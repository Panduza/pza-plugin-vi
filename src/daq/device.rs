use async_trait::async_trait;
use panduza_platform_core::{DriverOperations, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;
mod random_si_reader;

#[derive(Default)]
///
///
pub struct Device {}

#[async_trait]
impl DriverOperations for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        let interface = random_si_reader::RandomSiReader::default().into_arc_mutex();
        panduza_platform_core::std::class::acq_si::mount(
            "randommeter",
            "Pika",
            0.0,
            0xffff as f64,
            4,
            instance.clone(),
            interface.clone(),
        )
        .await?;
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
