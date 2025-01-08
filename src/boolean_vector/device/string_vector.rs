use std::fmt::Debug;

use panduza_platform_core::{
    format_settings_error, log_error, log_warn, Error, InstanceSettings, Logger, Props,
};
use serde_json::Value;

#[derive(Clone)]
pub struct Settings {
    ///
    ///
    pub logger: Option<Logger>,

    ///
    ///
    pub name: String,

    ///
    ///
    pub values: Vec<String>,
}

impl Settings {
    /// Build new instance
    ///
    pub fn new<A: Into<String>>(name: A, logger: Option<Logger>) -> Self {
        Self {
            logger: logger,
            name: name.into(),
            values: Vec::new(),
        }
    }

    ///
    ///
    pub fn override_with_instance_settings(
        &mut self,
        settings: &Option<InstanceSettings>,
    ) -> Result<(), Error> {
        //
        //
        if let Some(value) = settings {
            //
            //
            if value.is_object() {
                //
                //
                if let Some(map) = value.as_object() {
                    //
                    //
                    let value = map.get(&self.name).and_then(|v| {
                        v.as_array().and_then(|v| {
                            Some(
                                v.into_iter()
                                    .map(|value| match value {
                                        Value::String(s) => Ok(s),
                                        _ => {
                                            if let Some(logger) = &self.logger {
                                                log_warn!(self.logger, "{:?} not managed", value);
                                            }
                                        }
                                    })
                                    .collect(),
                            )
                        })
                    });
                    if let Some(v) = value {
                        if v >= self.default_min {
                            self.min = v;
                        } else {
                            if let Some(logger) = &self.logger {
                                log_error!(
                                    logger,
                                    "{} is lower than default {} < {}",
                                    &self.name,
                                    v,
                                    self.default_min
                                );
                            }
                            return Err(format_settings_error!(
                                "{} is lower than default {} < {}",
                                &self.name,
                                v,
                                self.default_min
                            ));
                        }
                    } else {
                        if let Some(logger) = &self.logger {
                            log_warn!(
                                logger,
                                "{} is not in settings, use default value {}",
                                &self.name,
                                self.default_min
                            );
                        }
                    }
                }
            } else {
                if let Some(logger) = &self.logger {
                    log_warn!(
                        logger,
                        "Instance settings is not an object for min/max {}",
                        &self.name
                    );
                }
            }
        } else {
            if let Some(logger) = &self.logger {
                log_warn!(logger, "No instance settings provided for {}", &self.name);
            }
        }

        Ok(())
    }

    /// Add props to props
    ///
    pub fn declare(&self, props: &mut Props) {
        // props.add_number_prop(
        //     &self.key,
        //     format!("Minimal {}", self.desc),
        //     self.default_min,
        // );
        // props.add_number_prop(
        //     &self.max_key,
        //     format!("Maximal {}", self.desc),
        //     self.default_max,
        // );
    }
}

impl Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("values", &self.values)
            .finish()
    }
}
