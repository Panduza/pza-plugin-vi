use panduza_platform_core::{Producer, Scanner};

#[cfg(feature = "plugin")]
panduza_platform_core::plugin_interface!("vi");

//
// Import modules
mod repl;
mod attribute_tester;

//
// Export the producers of the plugin
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(repl::Package::default().boxed());
    producers.push(attribute_tester::Package::default().boxed());
    return producers;
}

//
//
pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
    let scanners: Vec<Box<dyn Scanner>> = vec![];
    return scanners;
}
