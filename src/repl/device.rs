use async_trait::async_trait;
use panduza_platform_core::{DriverOperations, Error, Instance};
use std::time::Duration;
use tokio::time::sleep;
mod eval_echo;


#[derive(Default)]
///
/// Device to control PicoHA SSB Board
///
pub struct Device {}


#[async_trait]
impl DriverOperations for Device {
    ///
    ///
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        let driver = eval_echo::EvalEcho::default().into_arc_mutex();
        panduza_platform_core::std::class::repl::mount("repl", instance.clone(), driver).await?;
        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, mut _device: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
