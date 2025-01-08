mod controller;
mod settings;

use settings::Settings as ControlSettings;

pub mod string_vector;

use async_trait::async_trait;
use panduza_platform_core::{
    log_info, log_info_mount_end, log_info_mount_start, Container, DriverOperations, Error,
    Instance,
};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Default)]
///
///
pub struct Device {}

impl Device {
    ///
    ///
    pub async fn prepare_control_settings(
        &mut self,
        instance: Instance,
    ) -> Result<ControlSettings, Error> {
        //
        //
        let instance_settings = instance.settings().await;

        //
        //
        let mut control_settings = ControlSettings::new();
        control_settings.override_with_instance_settings(&instance_settings)?;

        //
        //
        Ok(control_settings)
    }
}

#[async_trait]
impl DriverOperations for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        //
        // Start logging
        let logger = instance.logger();
        log_info_mount_start!(logger);

        //
        //
        let control_settings = self.prepare_control_settings(instance.clone()).await?;
        log_info!(logger, "control_settings = {:?}", control_settings);

        for entry in control_settings.elements() {
            controller::mount(entry, instance.clone()).await?;
        }

        //
        // Ok
        log_info_mount_end!(logger);
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
