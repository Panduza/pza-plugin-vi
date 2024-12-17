use panduza_platform_core::{DriverOperations, Producer};
use super::device::ViDevice;

pub struct Vi {}

impl Vi {
    pub fn new() -> Box<Vi> {
        Box::new(Vi {})
    }
}

impl Producer for Vi {
    fn manufacturer(&self) -> String {
        "panduza".to_string()
    }

    fn model(&self) -> String {
        "VI".to_string()
    }

    fn description(&self) -> String {
        "Virtual Instrument interface".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(ViDevice::new()));
    }
}
