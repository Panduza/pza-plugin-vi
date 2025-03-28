use panduza_platform_core::{log_info, Container, Error, Instance};

//
//
pub async fn mount(mut instance: Instance, number_of_classes: i16) -> Result<(), Error> {
    //
    //
    for i in 1..=number_of_classes {
        //
        //
        let class_name = format!("bytes_{}", i);

        //
        // Create interface
        let mut class = instance.create_class(&class_name).finish().await;

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
        att_bytes_ro.set(vec![0; 16 * 1024]).await?;

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
        tokio::spawn({
            let att_bytes_ro = att_bytes_ro.clone();
            async move {
                loop {
                    att_bytes_wo.wait_for_commands().await;
                    while let Some(command) = att_bytes_wo.pop().await {
                        log_info!(att_bytes_wo.logger(), "command received - {:?}", command);
                        att_bytes_ro.set(command).await.unwrap();
                    }
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

        //
        //
        att_bytes_rw.set(vec![0; 16 * 1024]).await?;

        //
        //
        tokio::spawn({
            let mut att_bytes_rw = att_bytes_rw.clone();
            async move {
                loop {
                    att_bytes_rw.wait_for_commands().await;
                    while let Some(command) = att_bytes_rw.pop().await {
                        log_info!(att_bytes_rw.logger(), "command received - {:?}", command);
                        att_bytes_rw.set(command).await.unwrap();
                    }
                }
            }
        });
    }

    Ok(())
}
