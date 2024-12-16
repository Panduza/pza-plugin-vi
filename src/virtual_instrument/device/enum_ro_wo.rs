use panduza_platform_core::{spawn_on_command, Error, Instance, Class, EnumAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
    mut class: Class
) -> Result<(), Error> {
    //
    // Create interface
    // let mut c_interface = class.create_class("enum").finish();

    //
    //
    let att_enum_ro = c_interface
        .create_attribute("enum_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_enum(vec!["test_ro".to_string()])
        .await?;

    //
    //
    let att_enum_wo = c_interface
        .create_attribute("enum_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_enum(vec!["test_wo".to_string()])
        .await?;

    // 
    // 
    let att_enum_wo_2 = att_enum_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_enum_wo_2,
        on_command(
            att_enum_ro.clone(),
            att_enum_wo_2.clone()
        )
    );

    Ok(())
}

///
///
///
async fn on_command(
    mut att_enum_ro: EnumAttServer,
    att_enum_wo: EnumAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_enum_ro.pop_cmd().await {
        att_enum_wo.set(command?).await?;
    }

    Ok(())
}
