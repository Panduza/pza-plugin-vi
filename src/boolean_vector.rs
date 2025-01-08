pub mod device;
use panduza_platform_core::{std::prop, DriverOperations, Producer};

#[derive(Default)]
pub struct Package {}

impl Package {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Producer for Package {
    fn manufacturer(&self) -> String {
        "vi".to_string()
    }

    fn model(&self) -> String {
        "boolean_vector".to_string()
    }

    fn description(&self) -> String {
        "Virtual Instrument to emulate boolean signals".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        let mut props = panduza_platform_core::Props::default();
        // props. // list of string
        return props;
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(device::Device::default()));
    }
}
