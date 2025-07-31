use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::{Container, Error, Instance};

///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class(format!("enum")).finish().await;
    log_debug_mount_start!(class.logger());

    //
    // Some of the first contributors (sorted by alphabetic order)
    let choices = vec![
        "Adel".to_string(),
        "Antoine".to_string(),
        "Bryan".to_string(),
        "Damien".to_string(),
        "Edmundo".to_string(),
        "Florian".to_string(),
        "Lucas".to_string(),
        "Rethusan".to_string(),
        "Valentin".to_string(),
        "Xavier".to_string(),
    ];

    //
    //
    let att_enum_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_enum(choices.clone())
        .await?;
    att_enum_ro.set(choices.get(0).unwrap().clone()).await?;

    //
    //
    let mut att_enum_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_enum(choices.clone())
        .await?;

    //
    //
    let handler_att_enum_wo = tokio::spawn(async move {
        loop {
            if let Ok(command) = att_enum_wo.wait_for_commands().await {
                // log_info!(att_enum_wo.logger(), "command recieved - {:?}", command);
                att_enum_ro.set(command).await.unwrap();
            }
        }
    });

    instance
        .monitor_task("tester/enum/wo".to_string(), handler_att_enum_wo)
        .await;

    //
    //
    let mut att_enum_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_enum(choices.clone())
        .await?;
    att_enum_rw.set(choices.get(0).unwrap().clone()).await?;

    //
    //
    let handler_att_enum_rw = tokio::spawn(async move {
        loop {
            if let Ok(command) = att_enum_rw.wait_for_commands().await {
                log_info!(att_enum_rw.logger(), "command recieved - {:?}", command);
                att_enum_rw.set(command).await.unwrap();
            }
        }
    });

    instance
        .monitor_task("tester/enum/rw".to_string(), handler_att_enum_rw)
        .await;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
