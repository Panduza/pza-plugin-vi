use panduza_platform_core::{log_info, Container, Error, Instance};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("string").finish().await;

    //
    //
    let att_string_ro = class
        .create_attribute("string_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_string()
        .await?;

    //
    //
    att_string_ro.set("test".to_string()).await?;

    //
    //
    let att_string_wo = class
        .create_attribute("string_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_string()
        .await?;

    //
    //
    let att_string_wo_2 = att_string_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_string_wo_2,
        on_command(att_string_ro.clone(), att_string_wo_2.clone())
    );

    //
    //
    let att_string_rw = class
        .create_attribute("string_rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_string()
        .await?;
    att_string_rw.set("test".to_string()).await?;

    //
    //
    let att_string_rw_2 = att_string_rw.clone();
    spawn_on_command!(
        "on_command => string_rw",
        instance,
        att_string_rw_2,
        on_command_rw(att_string_rw_2.clone())
    );

    Ok(())
}

///
///
///
async fn on_command(
    att_string_ro: StringAttServer,
    mut att_string_wo: StringAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_string_wo.pop_cmd().await {
        log_info!(att_string_wo.logger(), "command recieved - {:?}", command);
        att_string_ro.set(command).await?;
    }

    Ok(())
}

///
///
///
async fn on_command_rw(mut att_string_rw: StringAttServer) -> Result<(), Error> {
    while let Some(command) = att_string_rw.pop_cmd().await {
        log_info!(att_string_rw.logger(), "command recieved - {:?}", command);
        att_string_rw.set(command).await?;
    }
    Ok(())
}
