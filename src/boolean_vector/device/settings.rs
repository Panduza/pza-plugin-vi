use panduza_platform_core::{Error, InstanceSettings};

use super::string_vector::Settings as StringVectorSettings;

///
///
#[derive(Debug, Clone)]
pub struct Settings {
    pub elements: StringVectorSettings,
}

impl Settings {
    ///
    ///
    pub fn new() -> Self {
        Self {
            elements: StringVectorSettings::new("elements", None),
        }
    }

    ///
    ///
    pub fn elements(&self) -> Vec<String> {
        self.elements.values.clone()
    }

    ///
    ///
    pub fn override_with_instance_settings(
        &mut self,
        settings: &Option<InstanceSettings>,
    ) -> Result<(), Error> {
        self.elements.override_with_instance_settings(settings)?;
        Ok(())
    }
}
