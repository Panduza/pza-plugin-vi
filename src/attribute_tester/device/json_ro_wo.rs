use panduza_platform_core::{
    log_info, spawn_on_command, Container, Error, Instance, JsonAttServer,
};
use serde_json::json;

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("json").finish().await;

    //
    //
    let att_json_ro = class
        .create_attribute("json_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_json()
        .await?;
    att_json_ro.set(json!({"test": 100})).await?;

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
        on_command(att_json_ro.clone(), att_json_wo_2.clone())
    );

    //
    //
    let att_json_rw = class
        .create_attribute("json_rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .finish_as_json()
        .await?;
    att_json_rw.set(json!({"test": 42})).await?;

    //
    //
    let att_json_rw_2 = att_json_rw.clone();
    spawn_on_command!(
        "on_command => json_rw",
        instance,
        att_json_rw_2,
        on_command_rw(att_json_rw_2.clone())
    );

    Ok(())
}

///
///
///
async fn on_command(
    att_json_ro: JsonAttServer,
    mut att_json_wo: JsonAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_json_wo.pop_cmd().await {
        log_info!(att_json_wo.logger(), "command recieved - {:?}", command);
        att_json_ro.set(command).await?;
    }
    Ok(())
}

///
///
///
async fn on_command_rw(mut att_json_rw: JsonAttServer) -> Result<(), Error> {
    while let Some(command) = att_json_rw.pop_cmd().await {
        log_info!(att_json_rw.logger(), "command recieved - {:?}", command);
        att_json_rw.set(command).await?;
    }
    Ok(())
}
