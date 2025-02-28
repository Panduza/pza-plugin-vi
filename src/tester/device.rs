mod boolean;
mod r#enum;
mod json;
mod string;
mod vector_f32;
mod waveform;
use async_trait::async_trait;
use panduza_platform_core::{Actions, Error, Instance};
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
impl Actions for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        string::mount(instance.clone()).await?;
        boolean::mount(instance.clone()).await?;
        waveform::mount(instance.clone()).await?;
        json::mount(instance.clone()).await?;
        vector_f32::mount(instance.clone()).await?;
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
