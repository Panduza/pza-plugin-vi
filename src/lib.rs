use panduza_platform_core::Producer;

#[cfg(feature = "plugin")]
panduza_platform_core::plugin_interface!("vi", "0.0.0");

//
// Import modules
// mod boolean_vector;
// mod daq;
mod platform_logger;
// mod repl;
pub mod tester;

//
// Export the producers of the plugin
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    // producers.push(repl::Package::default().boxed());
    // producers.push(daq::Package::default().boxed());
    producers.push(tester::Package::default().boxed());
    producers.push(platform_logger::Package::default().boxed());
    // producers.push(boolean_vector::Package::default().boxed());
    return producers;
}

//
//
// pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
//     let scanners: Vec<Box<dyn Scanner>> = vec![];
//     return scanners;
// }
