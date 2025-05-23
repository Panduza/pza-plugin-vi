use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::Container;
use panduza_platform_core::Error;
use panduza_platform_core::Instance;

/// This module contains the implementation of the boolean attribute test.
///
pub async fn mount(mut instance: Instance, overload: Option<usize>) -> Result<(), Error> {
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
    let handler_att_wo = tokio::spawn(async move {
        loop {
            att_boolean_wo.wait_for_commands().await;
            while let Some(command) = att_boolean_wo.pop().await {
                log_info!(att_boolean_wo.logger(), "command received - {:?}", command);
                att_boolean_ro.set(command).await.unwrap();
                log_info!(att_boolean_ro.logger(), "command replay - {:?}", command);
            }
        }
    });
    instance
        .monitor_task("tester/boolean/wo".to_string(), handler_att_wo)
        .await;

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
    let handler_att_rw = tokio::spawn(async move {
        loop {
            att_boolean_rw.wait_for_commands().await;
            while let Some(command) = att_boolean_rw.pop().await {
                log_info!(att_boolean_rw.logger(), "command received - {:?}", command);
                att_boolean_rw.set(command).await.unwrap();
            }
        }
    });
    instance
        .monitor_task("tester/boolean/rw".to_string(), handler_att_rw)
        .await;

    //
    // Create a write-only boolean attribute for alert simulation
    let mut att_boolean_alert = class
        .create_attribute("alert")
        .with_wo()
        .with_info(r#"# Alert Simulation Attribute

This attribute is used to simulate alert scenarios in the system. It is a write-only attribute, meaning its value can only be written to and not read directly.

## Purpose

- To test the system's behavior when alerts are triggered.
- To ensure proper handling of alert conditions.

## Example

- Initial value: `false`
- Expected behavior: Writing to this attribute triggers an alert for testing purposes.

### Additional Notes

- This attribute is intended for testing and debugging only.
- Use with caution as it will intentionally trigger an alert.
        "#)
        .start_as_boolean()
        .await?;

    //
    // Spawn a task to handle write-only attribute commands for alert simulation
    let handler_att_alert = tokio::spawn(async move {
        loop {
            att_boolean_alert.wait_for_commands().await;
            while let Some(_) = att_boolean_alert.pop().await {
                log_info!(att_boolean_alert.logger(), "Alert simulation triggered");
                att_boolean_alert
                    .trigger_alert("Simulated alert triggered for testing purposes")
                    .await;
            }
        }
    });
    instance
        .monitor_task("tester/boolean/alert".to_string(), handler_att_alert)
        .await;

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
    let handler_att_error = tokio::spawn(async move {
        loop {
            att_boolean_error.wait_for_commands().await;
            while let Some(_) = att_boolean_error.pop().await {
                log_info!(att_boolean_error.logger(), "Error simulation triggered");
                panic!("Simulated error triggered for testing purposes");
            }
        }
    });
    instance
        .monitor_task("tester/boolean/error".to_string(), handler_att_error)
        .await;

    //
    // if overload is set, create as may rw attributes as overload number
    if let Some(overload) = overload {
        for i in 0..overload {
            let mut att_boolean_overload = class
                .create_attribute(format!("overload_rw_{}", i))
                .with_rw()
                .with_info(&format!("Overload attribute number {}", i))
                .start_as_boolean()
                .await?;
            att_boolean_overload.set(false).await?;
            let handler_att_overload = tokio::spawn(async move {
                loop {
                    att_boolean_overload.wait_for_commands().await;
                    while let Some(command) = att_boolean_overload.pop().await {
                        log_info!(
                            att_boolean_overload.logger(),
                            "command received - {:?}",
                            command
                        );
                        att_boolean_overload.set(command).await.unwrap();
                    }
                }
            });
            instance
                .monitor_task(
                    format!("tester/boolean/overload_rw_{}", i),
                    handler_att_overload,
                )
                .await;
        }
    }

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
