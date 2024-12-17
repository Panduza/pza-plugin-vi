use panduza_platform_core::{log_info, spawn_on_command, Error, Instance, InstanceLogger, StringAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("string").finish();

    //
    //
    let att_string_ro = class
        .create_attribute("string_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_string()
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
        .finish_as_string()
        .await?;

    // 
    // 
    att_string_wo.set("test".to_string()).await?;

    // 
    // 
    let att_string_wo_2 = att_string_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_string_wo_2,
        on_command(
            att_string_ro.clone(),
            att_string_wo_2.clone()
        )
    );
    
    // let Some(resp) = att_string_ro.pop_cmd().await;
    // att_string_wo.set(resp).await?;

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
        att_string_ro.set(command).await?;
    }

    Ok(())
}
