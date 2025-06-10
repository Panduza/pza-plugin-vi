use bytes::Bytes;
use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::{Container, Error, Instance};

///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("bytes").finish().await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_bytes_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_bytes()
        .await?;

    //
    //
    att_bytes_ro.set(Bytes::from("initial")).await?;

    //
    //
    let mut att_bytes_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_bytes()
        .await?;

    //
    //
    let handler_att_bytes_wo = tokio::spawn(async move {
        loop {
            if let Ok(command) = att_bytes_wo.wait_for_commands().await {
                // log_info!(att_bytes_wo.logger(), "command recieved - {:?}", command);
                att_bytes_ro.set(command).await.unwrap();
            }
        }
    });
    instance
        .monitor_task("tester/bytes/wo".to_string(), handler_att_bytes_wo)
        .await;

    //
    //
    let mut att_bytes_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_bytes()
        .await?;
    att_bytes_rw.set(Bytes::from("initial")).await?;

    //
    //
    let handler_att_bytes_rw = tokio::spawn(async move {
        loop {
            if let Ok(command) = att_bytes_rw.wait_for_commands().await {
                log_info!(att_bytes_rw.logger(), "command recieved - {:?}", command);
                att_bytes_rw.set(command).await.unwrap();
            }
        }
    });
    instance
        .monitor_task("tester/bytes/rw".to_string(), handler_att_bytes_rw)
        .await;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
