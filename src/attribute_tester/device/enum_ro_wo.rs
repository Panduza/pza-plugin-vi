use panduza_platform_core::{
    log_error, log_info, spawn_on_command, Container, EnumAttServer, Error, Instance,
};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("enum").finish().await;

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
        .create_attribute("enum_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_enum(choices.clone())
        .await?;
    att_enum_ro.set(choices.get(0).unwrap().clone()).await?;

    //
    //
    let att_enum_wo = class
        .create_attribute("enum_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_enum(choices.clone())
        .await?;

    //
    //
    let att_enum_wo_2 = att_enum_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_enum_wo_2,
        on_command(att_enum_ro.clone(), att_enum_wo_2.clone())
    );

    //
    //
    let att_enum_rw = class
        .create_attribute("enum_rw")
        .with_wo()
        .with_info(r#"read write command"#)
        .finish_as_enum(choices.clone())
        .await?;
    att_enum_rw.set(choices.get(0).unwrap().clone()).await?;

    //
    //
    let att_enum_rw_2 = att_enum_rw.clone();
    spawn_on_command!(
        "on_command => enum_rw",
        instance,
        att_enum_rw_2,
        on_command_rw(att_enum_rw_2.clone())
    );

    Ok(())
}

///
///
///
async fn on_command(
    att_enum_ro: EnumAttServer,
    mut att_enum_wo: EnumAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_enum_wo.pop_cmd().await {
        match command {
            Ok(c) => {
                log_info!(att_enum_ro.logger(), "command recieved - {:?}", c);
                att_enum_ro.set(c).await?;
            }
            Err(e) => {
                log_error!(att_enum_ro.logger(), "command recieved err - {:?}", e);
            }
        }
    }

    Ok(())
}

///
///
///
async fn on_command_rw(mut att_enum_rw: EnumAttServer) -> Result<(), Error> {
    while let Some(command) = att_enum_rw.pop_cmd().await {
        match command {
            Ok(c) => {
                log_info!(att_enum_rw.logger(), "command recieved - {:?}", c);
                att_enum_rw.set(c).await?;
            }
            Err(e) => {
                log_error!(att_enum_rw.logger(), "command recieved err - {:?}", e);
            }
        }
    }
    Ok(())
}
