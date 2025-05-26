use panduza_platform_core::{log_info, Container, Error, Instance};

//
//
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("counter").finish().await;
    //
    //
    let att_number_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_number()
        .await?;
    //
    //
    att_number_ro.set(0).await?;

    //
    //
    let mut att_number_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_number()
        .await?;

    //
    //
    tokio::spawn(async move {
        loop {
            if let Ok(command) = att_number_wo.wait_for_commands().await {
                // log_info!(att_number_wo.logger(), "command recieved - {:?}", command);
                att_number_ro.set(command).await.unwrap();
            }
        }
    });

    //
    //
    let mut att_number_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_number()
        .await?;

    //
    //
    tokio::spawn(async move {
        loop {
            if let Ok(command) = att_number_rw.wait_for_commands().await {
                log_info!(att_number_rw.logger(), "command recieved - {:?}", command);
                att_number_rw.set(command).await.unwrap();
            }
        }
    });

    Ok(())
}
