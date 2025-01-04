use async_trait::async_trait;
use panduza_platform_core::{log_info, spawn_loop, Container, DriverOperations, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;

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
        let mut instance_2 = instance.clone();
        spawn_loop!("test", instance_2, {
            let counter: u64 = 0;
            log_info!(instance.logger(), "Hello {}", counter);
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
