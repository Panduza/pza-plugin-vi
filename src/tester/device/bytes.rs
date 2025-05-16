use bytes::Bytes;
use panduza_platform_core::{log_info, Container, Error, Instance};

///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("bytes").finish().await;

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
    tokio::spawn(async move {
        loop {
            att_bytes_wo.wait_for_commands().await;
            while let Some(command) = att_bytes_wo.pop().await {
                log_info!(att_bytes_wo.logger(), "command recieved - {:?}", command);
                att_bytes_ro.set(command).await.unwrap();
            }
        }
    });

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
    tokio::spawn(async move {
        loop {
            att_bytes_rw.wait_for_commands().await;
            while let Some(command) = att_bytes_rw.pop().await {
                log_info!(att_bytes_rw.logger(), "command recieved - {:?}", command);
                att_bytes_rw.set(command).await.unwrap();
            }
        }
    });

    Ok(())
}
