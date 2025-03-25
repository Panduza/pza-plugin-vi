use panduza_platform_core::{log_info, Container, Error, Instance};

///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("string").finish().await;

    //
    //
    let att_string_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_string()
        .await?;

    //
    //
    att_string_ro.set("test".to_string()).await?;

    //
    //
    let mut att_string_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_string()
        .await?;

    //
    //
    tokio::spawn(async move {
        loop {
            att_string_wo.wait_for_commands().await;
            while let Some(command) = att_string_wo.pop().await {
                log_info!(att_string_wo.logger(), "command recieved - {:?}", command);
                att_string_ro.set(command).await.unwrap();
            }
        }
    });

    //
    //
    let mut att_string_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_string()
        .await?;
    att_string_rw.set("test".to_string()).await?;

    //
    //
    tokio::spawn(async move {
        loop {
            att_string_rw.wait_for_commands().await;
            while let Some(command) = att_string_rw.pop().await {
                log_info!(att_string_rw.logger(), "command recieved - {:?}", command);
                att_string_rw.set(command).await.unwrap();
            }
        }
    });

    Ok(())
}
