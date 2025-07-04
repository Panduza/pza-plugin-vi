mod boolean;
mod bytes;
// mod r#enum;
mod json;
mod number;
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
        let boolean_overload = if let Some(settings) = instance.settings().await {
            settings
                .get("boolean_overload")
                .and_then(|v| v.as_u64().map(|n| n as usize))
        } else {
            None
        };

        boolean::mount(instance.clone(), boolean_overload).await?;
        number::mount(instance.clone()).await?;
        string::mount(instance.clone()).await?;
        bytes::mount(instance.clone()).await?;

        // json::mount(instance.clone()).await?;
        // vector_f32::mount(instance.clone()).await?;
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
