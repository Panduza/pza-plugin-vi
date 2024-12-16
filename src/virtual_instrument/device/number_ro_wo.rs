use panduza_platform_core::{log_info, spawn_on_command, Error, Instance, InstanceLogger, NumberAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
) -> Result<(), Error> {
    //
    // 
    let logger = instance.logger.clone();
    logger.info("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    //
    // Create interface
    let mut class = instance.create_class("number").finish();

    //
    //
    let att_number_ro = class
        .create_attribute("number_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_number()
        .await?;

    // 
    // 
    att_number_ro.set_from_i64(0);

    //
    //
    let att_number_wo = class
        .create_attribute("number_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_number()
        .await?;
        
    // 
    // 
    att_number_wo.set_from_i64(0);

    // 
    // 
    let logger_2 = logger.clone();
    let att_number_wo_2 = att_number_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_number_wo_2,
        on_command(
            logger_2.clone(),
            att_number_ro.clone(),
            att_number_wo_2.clone()
        )
    );
    
    logger.info("fin numberrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr");

    Ok(())
}

///
///
///
async fn on_command(
    logger: InstanceLogger,
    att_number_ro: NumberAttServer,
    mut att_number_wo: NumberAttServer,
) -> Result<(), Error> {
    log_info!(logger, "commmmmmmmmmmmmmannnnnnnnnnnnnnnnnnnnnndddddddddddddddddddddd");
    while let Some(command) = att_number_wo.pop_cmd_as_i64().await {
        log_info!(logger, "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz");
        att_number_ro.set_from_i64(command).await?;
        att_number_wo.set_from_i64(command).await?;
    }

    Ok(())
}
