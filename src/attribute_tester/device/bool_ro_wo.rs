use panduza_platform_core::{
    log_info, spawn_on_command, BooleanAttServer, Container, Error, Instance,
};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("boolean").finish().await;

    //
    //
    let att_boolean_ro = class
        .create_attribute("boolean_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_boolean()
        .await?;
    att_boolean_ro.set(false).await?;

    //
    //
    let att_boolean_wo = class
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
        on_command(att_boolean_ro.clone(), att_boolean_wo_2.clone())
    );

    //
    //
    let att_boolean_rw = class
        .create_attribute("boolean_rw")
        .with_wo()
        .with_info(r#"read write command"#)
        .finish_as_boolean()
        .await?;
    att_boolean_rw.set(false).await?;

    //
    //
    let att_boolean_rw_2 = att_boolean_rw.clone();
    spawn_on_command!(
        "on_command => boolean_rw",
        instance,
        att_boolean_rw_2,
        on_command_rw(att_boolean_rw_2.clone())
    );

    Ok(())
}

///
///
///
async fn on_command(
    att_boolean_ro: BooleanAttServer,
    mut att_boolean_wo: BooleanAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_boolean_wo.pop_cmd().await {
        log_info!(att_boolean_wo.logger(), "command recieved - {:?}", command);
        att_boolean_ro.set(command).await?;
    }
    Ok(())
}

///
///
///
async fn on_command_rw(mut att_boolean_rw: BooleanAttServer) -> Result<(), Error> {
    while let Some(command) = att_boolean_rw.pop_cmd().await {
        log_info!(att_boolean_rw.logger(), "command recieved - {:?}", command);
        att_boolean_rw.set(command).await?;
    }
    Ok(())
}
