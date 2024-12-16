// mod string_ro_wo;
// mod string_rw;
// mod bool_ro_wo;
// mod enum_ro_wo;
// mod json_ro_wo;
// mod number_ro_wo;
// mod si_ro_wo;

// use panduza_platform_core::{Error, Instance};

// ///
// ///
// ///
// pub async fn mount(mut instance: Instance) -> Result<(), Error> {
//     // 
//     // 
//     let itf_type = instance.create_class("type").finish();

//     string_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
//     string_rw::mount(instance.clone(), itf_type.clone()).await?;
//     bool_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
//     enum_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
//     json_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
//     number_ro_wo::mount(instance.clone(), itf_type.clone()).await?;
//     si_ro_wo::mount(instance.clone(), itf_type.clone()).await?;


//     Ok(())
// }