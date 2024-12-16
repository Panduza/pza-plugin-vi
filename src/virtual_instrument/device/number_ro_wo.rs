use panduza_platform_core::{spawn_on_command, Error, Instance, Class, NumberAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
    mut class: Class
) -> Result<(), Error> {
    //
    // Create interface
    // let mut c_interface = class.create_class("number").finish();

    //
    //
    let att_number_ro = c_interface
        .create_attribute("number_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_number()
        .await?;

    //
    //
    let att_number_wo = c_interface
        .create_attribute("number_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_number()
        .await?;

    // 
    // 
    let att_number_wo_2 = att_number_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_number_wo_2,
        on_command(
            att_number_ro.clone(),
            att_number_wo_2.clone()
        )
    );
    
    Ok(())
}

///
///
///
async fn on_command(
    mut att_number_ro: NumberAttServer,
    att_number_wo: NumberAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_number_ro.pop_cmd_as_i64().await {
        att_number_wo.set_from_i64(command).await?;
    }

    Ok(())
}
