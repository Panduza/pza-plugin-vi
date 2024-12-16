use panduza_platform_core::{spawn_on_command, Error, Instance, Class, BooleanAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
    mut class: Class
) -> Result<(), Error> {
    //
    // Create interface
    // let mut c_interface = class.create_class("boolean").finish();

    //
    //
    let att_boolean_ro = c_interface
        .create_attribute("boolean_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_boolean()
        .await?;

    //
    //
    let att_boolean_wo = c_interface
        .create_attribute("boolean_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_boolean()
        .await?;

    // 
    // 
    let att_boolean_wo_2 = att_boolean_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_boolean_wo_2,
        on_command(
            att_boolean_ro.clone(),
            att_boolean_wo_2.clone()
        )
    );
    Ok(())
}

///
///
///
async fn on_command(
    mut att_boolean_ro: BooleanAttServer,
    att_boolean_wo: BooleanAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_boolean_ro.pop_cmd().await {
        att_boolean_wo.set(command).await?;
    }

    Ok(())
}
