mod datatype;

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
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        
        datatype::mount(instance.clone()).await?;
        
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
