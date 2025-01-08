use panduza_platform_core::{
    log_debug, log_debug_mount_end, log_debug_mount_start, spawn_on_command, BooleanAttServer,
    Container, Error,
};

///
///
pub async fn mount<A: Into<String>, C: Container + 'static>(
    name: A,
    mut parent: C,
) -> Result<(), Error> {
    //
    //
    let att = parent
        .create_attribute(name)
        .with_rw()
        .finish_as_boolean()
        .await?;
    let logger = att.logger().clone();
    log_debug_mount_start!(logger);

    //
    spawn_on_command!(
        "on_command => boolean",
        parent,
        att,
        on_command(att.clone())
    );

    //
    //
    log_debug_mount_end!(logger);
    Ok(())
}

///
///
async fn on_command(mut att: BooleanAttServer) -> Result<(), Error> {
    while let Some(command) = att.pop_cmd().await {
        //
        // Log
        log_debug!(att.logger(), "OCP command received '{:?}'", command);

        //
        //
        att.set(command).await?;
    }
    Ok(())
}
