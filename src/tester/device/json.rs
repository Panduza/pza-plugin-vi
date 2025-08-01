use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::{log_info, Container, Error, Instance};
use serde_json::json;

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("json").finish().await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_json_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_json()
        .await?;
    att_json_ro.set(json!({"test": 100})).await?;

    //
    //
    let mut att_json_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_json()
        .await?;

    //
    //
    let handler_att_json_wo = tokio::spawn(async move {
        loop {
            if let Ok(command) = att_json_wo.wait_for_commands().await {
                // log_info!(att_json_wo.logger(), "command recieved - {:?}", command);
                att_json_ro.set(command).await.unwrap();
            }
        }
    });

    instance
        .monitor_task("tester/json/wo".to_string(), handler_att_json_wo)
        .await;

    //
    //
    let mut att_json_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_json()
        .await?;
    att_json_rw.set(json!({"test": 42})).await?;

    //
    //
    let handler_att_json_rw = tokio::spawn(async move {
        loop {
            if let Ok(command) = att_json_rw.wait_for_commands().await {
                log_info!(att_json_rw.logger(), "command recieved - {:?}", command);
                att_json_rw.set(command).await.unwrap();
            }
        }
    });

    instance
        .monitor_task("tester/json/rw".to_string(), handler_att_json_rw)
        .await;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
