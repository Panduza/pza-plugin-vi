use panduza_platform_core::{spawn_on_command, Error, Instance, JsonAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("json").finish();

    //
    //
    let att_json_ro = class
        .create_attribute("json_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_json()
        .await?;

    //
    //
    let att_json_wo = class
        .create_attribute("json_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_json()
        .await?;

    // 
    // 
    let att_json_wo_2 = att_json_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_json_wo_2,
        on_command(
            att_json_ro.clone(),
            att_json_wo_2.clone()
        )
    );

    Ok(())
}

///
///
///
async fn on_command(
    mut att_json_ro: JsonAttServer,
    att_json_wo: JsonAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_json_ro.pop_cmd().await {
        att_json_wo.set(command).await?;
    }

    Ok(())
}
