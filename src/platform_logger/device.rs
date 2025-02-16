use async_trait::async_trait;
use panduza_platform_core::{log_info, spawn_loop, Actions, Container, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Default)]
///
///
pub struct Device {}

#[async_trait]
impl Actions for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        let mut instance_2 = instance.clone();
        // spawn_loop!("test", instance_2, {
        //     let mut counter: u64 = 0;
        //     loop {
        //         log_info!(instance.logger(), "Hello {}", counter);
        //         tokio::time::sleep(Duration::from_secs(1)).await;
        //         counter += 1;
        //     }
        // });
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
