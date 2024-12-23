use panduza_platform_core::{
    log_error, log_info, spawn_on_command, Container, Error, Instance, SiAttServer,
};

///
///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("si").finish().await;

    //
    //
    let att_si_ro = class
        .create_attribute("si_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .finish_as_si("test", 0.0, 100.0, 2)
        .await?;
    att_si_ro.set_from_f32(0.0).await?;

    //
    //
    let att_si_wo = class
        .create_attribute("si_wo")
        .with_wo()
        .with_info(r#"write command"#)
        .finish_as_si("test", 0.0, 100.0, 2)
        .await?;

    //
    //
    let att_si_wo_2 = att_si_wo.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_si_wo_2,
        on_command(att_si_ro.clone(), att_si_wo_2.clone())
    );

    //
    //
    let att_si_rw = class
        .create_attribute("si_rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .finish_as_si("test", 0.0, 100.0, 2)
        .await?;
    att_si_rw.set_from_f32(0.0).await?;

    //
    //
    let att_si_rw_2 = att_si_rw.clone();
    spawn_on_command!(
        "on_command => si_rw",
        instance,
        att_si_rw_2,
        on_command_rw(att_si_rw_2.clone())
    );

    Ok(())
}

///
///
///
async fn on_command(att_si_ro: SiAttServer, mut att_si_wo: SiAttServer) -> Result<(), Error> {
    while let Some(command) = att_si_wo.pop_cmd_as_f32().await {
        match command {
            Ok(c) => {
                log_info!(att_si_wo.logger(), "command recieved - {:?}", command);
                att_si_ro.set_from_f32(c).await?;
            }
            Err(e) => {
                log_error!(att_si_wo.logger(), "command recieved error - {:?}", e);
            }
        }
    }

    Ok(())
}

///
///
///
async fn on_command_rw(mut att_si_rw: SiAttServer) -> Result<(), Error> {
    while let Some(command) = att_si_rw.pop_cmd_as_f32().await {
        match command {
            Ok(c) => {
                log_info!(att_si_rw.logger(), "command recieved - {:?}", command);
                att_si_rw.set_from_f32(c).await?;
            }
            Err(e) => {
                log_error!(att_si_rw.logger(), "command recieved error - {:?}", e);
            }
        }
    }
    Ok(())
}
