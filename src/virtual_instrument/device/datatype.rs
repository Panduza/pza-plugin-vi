mod dstring;

use panduza_platform_core::{Error, Instance};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    // 
    // 
    let itf_type = instance.create_class("type").finish();

    dstring::mount(instance.clone(), itf_type.clone()).await?;

    Ok(())
}