use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

/// This module contains the implementation of the boolean attribute test.
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("boolean").finish().await;
    log_debug_mount_start!(class.logger());

    //
    // Create a read-only boolean attribute
    let att_boolean_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"# Boolean Attribute Test

This attribute is used to test boolean values in the system. It is a read-only attribute, meaning its value can only be read and not modified directly.

## Purpose

- To verify the behavior of boolean attributes.
- To ensure the system handles `true` and `false` values correctly.

## Example

- Initial value: `false`
- Expected behavior: The value changes based on external triggers or commands.
        "#)
        .start_as_boolean()
        .await?;
    att_boolean_ro.set(false).await?;

    //
    // Create a write-only boolean attribute
    let mut att_boolean_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"# Boolean Attribute Test

This attribute is used to test boolean values in the system. It is a write-only attribute, meaning its value can only be written to and not read directly.

## Purpose

- To verify the behavior of boolean attributes.
- To ensure the system handles `true` and `false` values correctly.

## Example

- Initial value: `false`
- Expected behavior: The value changes based on external triggers or commands.
        "#)
        .start_as_boolean()
        .await?;

    //
    // Spawn a task to handle write-only attribute commands
    tokio::spawn(async move {
        loop {
            att_boolean_wo.wait_for_commands().await;
            while let Some(command) = att_boolean_wo.pop().await {
                log_info!(att_boolean_wo.logger(), "command received - {:?}", command);
                att_boolean_ro.set(command).await.unwrap();
            }
        }
    });

    //
    // Create a read-write boolean attribute
    let mut att_boolean_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(
            r#"# Read Write Command

This attribute is used to test boolean values in the system. It is a read-write attribute, meaning its value can be both read and modified.

## Purpose

- To verify the behavior of read-write boolean attributes.
- To ensure the system handles `true` and `false` values correctly.

## Example

- Initial value: `false`
- Expected behavior: The value can be read and updated as needed.

### Additional Notes

- This attribute supports both reading and writing operations.
- Ensure proper synchronization when modifying the value.
            "#,
        )
        .start_as_boolean()
        .await?;
    att_boolean_rw.set(false).await?;

    //
    // Spawn a task to handle read-write attribute commands
    tokio::spawn(async move {
        loop {
            att_boolean_rw.wait_for_commands().await;
            while let Some(command) = att_boolean_rw.pop().await {
                log_info!(att_boolean_rw.logger(), "command received - {:?}", command);
                att_boolean_rw.set(command).await.unwrap();
            }
        }
    });

    //
    // Create a write-only boolean attribute for error simulation
    let mut att_boolean_error = class
        .create_attribute("error")
        .with_wo()
        .with_info(r#"# Error Simulation Attribute

This attribute is used to simulate error scenarios in the system. It is a write-only attribute, meaning its value can only be written to and not read directly.

## Purpose

- To test the system's behavior when errors are triggered.
- To ensure proper handling of unexpected conditions.

## Example

- Initial value: `false`
- Expected behavior: Writing to this attribute triggers an error for testing purposes.

### Additional Notes

- This attribute is intended for testing and debugging only.
- Use with caution as it will intentionally cause a panic.
        "#)
        .start_as_boolean()
        .await?;

    //
    // Spawn a task to handle write-only attribute commands for error simulation
    tokio::spawn(async move {
        loop {
            att_boolean_error.wait_for_commands().await;
            while let Some(_) = att_boolean_error.pop().await {
                log_info!(att_boolean_error.logger(), "Error simulation triggered");
                panic!("Simulated error triggered for testing purposes");
            }
        }
    });

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
