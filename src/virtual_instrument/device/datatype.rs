mod string_ro_wo;
mod string_rw;

use panduza_platform_core::{Error, Instance};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    // 
    // 
    let itf_type = instance.create_class("type").finish();

    string_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
    string_rw::mount(instance.clone(), itf_type.clone()).await?;

    Ok(())
}