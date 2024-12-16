use panduza_platform_core::{spawn_on_command, Error, Instance, Class, SiAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
    mut class: Class
) -> Result<(), Error> {
    //
    // Create interface
    // let mut c_interface = class.create_class("si").finish();

    //
    //
    let att_si_ro = c_interface
        .create_attribute("si_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_si("test", 0, 0, 1)
        .await?;

    //
    //
    let att_si_wo = c_interface
        .create_attribute("si_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_si("test", 0, 0, 1)
        .await?;

    // 
    // 
    let att_si_wo_2 = att_si_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_si_wo_2,
        on_command(
            att_si_ro.clone(),
            att_si_wo_2.clone()
        )
    );

    Ok(())
}

///
///
///
async fn on_command(
    mut att_si_ro: SiAttServer,
    att_si_wo: SiAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_si_ro.pop_cmd_as_f32().await {
        att_si_wo.set_from_f32(command?).await?;
    }

    Ok(())
}
