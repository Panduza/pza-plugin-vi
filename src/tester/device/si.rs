use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::NumberBuffer;
use panduza_platform_core::{log_info, Container, Error, Instance}; // Import NumberBuffer

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("si").finish().await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_si_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_si("Ω", -20.0, 100000.0, 2)
        .await?;
    att_si_ro.set(NumberBuffer::from(0.0)).await?;

    //
    //
    let mut att_si_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_si("Ω", -20.0, 100000.0, 2)
        .await?;

    // let handler_att_si_wo = tokio::spawn(async move {
    //     loop {
    //         if let Ok(command) = att_si_wo.wait_for_commands().await {
    //             // log_info!(att_si_wo.logger(), "command received - {:?}", command);
    //             att_si_ro.set(command).await.unwrap();
    //         }
    //     }
    // });

    // instance
    //     .monitor_task("tester/si/wo".to_string(), handler_att_si_wo)
    //     .await;

    //
    //
    let mut att_si_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_si("Ω", -20.0, 100000.0, 2)
        .await?;
    att_si_rw.set(NumberBuffer::from(0.0)).await?;

    // let handler_att_si_rw = tokio::spawn(async move {
    //     loop {
    //         if let Ok(command) = att_si_rw.wait_for_commands().await {
    //             log_info!(att_si_rw.logger(), "command received - {:?}", command);
    //             att_si_rw.set(command).await.unwrap();
    //         }
    //     }
    // });

    // instance
    //     .monitor_task("tester/si/rw".to_string(), handler_att_si_rw)
    //     .await;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
