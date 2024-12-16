// mod datatype;
mod string_ro_wo;
mod string_rw;
mod bool_ro_wo;
mod enum_ro_wo;
mod json_ro_wo;
mod number_ro_wo;
mod si_ro_wo;

use async_trait::async_trait;
use panduza_platform_core::{DriverOperations, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;

///
/// 
///
pub struct ViDevice {}

impl ViDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        ViDevice {}
    }
}

#[async_trait]
impl DriverOperations for ViDevice {
    ///
    ///
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        
        let itf_type = instance.create_class("type").finish();

        // datatype::mount(instance.clone()).await?;
        string_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
        string_rw::mount(instance.clone(), itf_type.clone()).await?;
        bool_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
        enum_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
        json_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
        number_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
        si_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
        
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
