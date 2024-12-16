use panduza_platform_core::{spawn_on_command, Error, Instance, Class, StringAttServer};

///
///
///
pub async fn mount(
    mut instance: Instance,
    mut class: Class
) -> Result<(), Error> {
    //
    // Create interface
    // let mut c_interface = class.create_class("string").finish();

    //
    //
    let att_string_rw = c_interface
        .create_attribute("string_ro")
        .with_rw()
        .with_info(r#"read command"#)
        .finish_as_string()
        .await?;

    // 
    // 
    let att_string_rw_2 = att_string_rw.clone();
    spawn_on_command!(
        "on_command",
        instance,
        att_string_rw_2,
        on_command(
            att_string_rw_2.clone()
        )
    );

    Ok(())
}

///
///
///
async fn on_command(
    mut att_string_rw: StringAttServer,
) -> Result<(), Error> {
    while let Some(command) = att_string_rw.pop_cmd().await {
        att_string_rw.set(command).await?;
    }

    Ok(())
}
