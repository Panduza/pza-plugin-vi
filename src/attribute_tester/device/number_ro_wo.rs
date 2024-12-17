use panduza_platform_core::{
    log_info, spawn_on_command, Container, Error, Instance, NumberAttServer,
};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("number").finish().await;

    //
    //
    let att_number_ro = class
        .create_attribute("number_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_number()
        .await?;

    //
    //
    att_number_ro.set_from_i64(0).await?;

    //
    //
    let att_number_wo = class
        .create_attribute("number_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_number()
        .await?;

    //
    //
    let att_number_wo_2 = att_number_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_number_wo_2,
        on_command(att_number_ro.clone(), att_number_wo_2.clone())
    );

    //
    //
    let att_number_rw = class
        .create_attribute("number_rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .finish_as_number()
        .await?;
    att_number_rw.set_from_i64(0).await?;

    //
    //
    let att_number_rw_2 = att_number_rw.clone();
    spawn_on_command!(
        "on_command => number_rw",
        instance,
        att_number_rw_2,
        on_command_rw(att_number_rw_2.clone())
    );

    Ok(())
}

///
///
///
async fn on_command(
    att_number_ro: NumberAttServer,
    mut att_number_wo: NumberAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_number_wo.pop_cmd_as_i64().await {
        log_info!(att_number_wo.logger(), "command recieved - {:?}", command);
        att_number_ro.set_from_i64(command).await?;
    }

    Ok(())
}

///
///
///
async fn on_command_rw(mut att_number_rw: NumberAttServer) -> Result<(), Error> {
    while let Some(command) = att_number_rw.pop_cmd_as_i64().await {
        log_info!(att_number_rw.logger(), "command recieved - {:?}", command);
        att_number_rw.set_from_i64(command).await?;
    }
    Ok(())
}
