use futures::FutureExt;
use panduza_platform_core::instance::server::boolean::BooleanAttributeServer;
use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::Container;
use panduza_platform_core::Error;
use panduza_platform_core::Instance;
use panduza_platform_core::NumberBuffer;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    // Create a counter to track wo commands
    let att_wo_counter = class
        .create_attribute("wo_counter")
        .with_ro()
        .with_info(
            r#"# WO Command Counter

This attribute tracks the number of commands received by the wo (write-only) boolean attribute.

## Purpose
- To count how many commands are sent to the write-only attribute.
- To provide metrics for testing purposes.

## Example
- Initial value: 0
- Value increments each time a command is received by the wo attribute.
"#,
        )
        .start_as_si("", 0.0, 1000000.0, 0) // Using integers, no decimals
        .await?;
    att_wo_counter.set(NumberBuffer::from(0.0)).await?;

    // Create a shared command counter
    let wo_command_counter = Arc::new(Mutex::new(0));

    //
    // Create a boolean attribute to reset the counter
    let mut att_wo_counter_reset = class
        .create_attribute("wo_counter_reset")
        .with_wo()
        .with_info(
            r#"# WO Counter Reset

This attribute resets the command counter for the wo (write-only) boolean attribute.

## Purpose
- To reset the counter to zero when needed.
- To provide testing control over the counter state.

## Example
- Send any boolean value to this attribute to reset the counter to 0.
- After reset, the wo_counter attribute will be set back to 0.
"#,
        )
        .start_as_boolean()
        .await?;

    // Ajout du callback pour reset le compteur
    // {
    //     let counter_reset_clone = wo_command_counter.clone();
    //     let att_wo_counter = Arc::new(att_wo_counter);
    //     let att_wo_counter_reset = att_wo_counter_reset.clone();
    //     att_wo_counter_reset
    //         .add_callback(
    //             move |_payload| {
    //                 let counter_reset_clone = counter_reset_clone.clone();
    //                 let att_wo_counter = att_wo_counter.clone();
    //                 let att_wo_counter_reset = att_wo_counter_reset.clone();
    //                 async move {
    //                     let mut counter = counter_reset_clone.lock().await;
    //                     *counter = 0;
    //                     att_wo_counter.set(NumberBuffer::from(0.0)).await.unwrap();
    //                     log_info!(att_wo_counter_reset.logger(), "Counter reset to 0");
    //                 }
    //                 .boxed()
    //             },
    //             None::<fn(&_) -> bool>,
    //         )
    //         .await;
    // }

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

    // Ajout du callback pour incrémenter le compteur et mettre à jour la valeur RO
    // {
    //     let counter_clone = wo_command_counter.clone();
    //     let att_wo_counter = Arc::new(att_wo_counter);
    //     let att_boolean_ro = att_boolean_ro.clone();
    //     att_boolean_wo
    //         .add_callback(
    //             move |payload| {
    //                 let counter_clone = counter_clone.clone();
    //                 let att_wo_counter = att_wo_counter.clone();
    //                 let att_boolean_ro = att_boolean_ro.clone();
    //                 async move {
    //                     let mut counter = counter_clone.lock().await;
    //                     *counter += 1;
    //                     att_wo_counter
    //                         .set(NumberBuffer::from(*counter as f64))
    //                         .await
    //                         .unwrap();
    //                     att_boolean_ro.set(payload).await.unwrap();
    //                 }
    //                 .boxed()
    //             },
    //             None::<fn(&_) -> bool>,
    //         )
    //         .await;
    // }

    //
    // Create a read-write boolean attribute
    create_rw_boolean_attribute(&mut class).await?;

    //
    // Create a write-only boolean attribute pour alert
    create_alert_boolean_attribute(&mut class).await?;

    //
    // Create a write-only boolean attribute pour error
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

    // Ajout du callback pour la simulation d'erreur
    // {
    //     let att_boolean_error = att_boolean_error.clone();
    //     att_boolean_error
    //         .add_callback(
    //             move |_payload| {
    //                 let att_boolean_error = att_boolean_error.clone();
    //                 async move {
    //                     log_info!(att_boolean_error.logger(), "Error simulation triggered");
    //                     panic!("Simulated error triggered for testing purposes");
    //                 }
    //                 .boxed()
    //             },
    //             None::<fn(&_) -> bool>,
    //         )
    //         .await;
    // }

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
            // Ajout du callback pour chaque overload
            {
                let att_boolean_overload = att_boolean_overload.clone();
                // att_boolean_overload
                //     .add_callback(
                //         move |command| {
                //             let att_boolean_overload = att_boolean_overload.clone();
                //             async move {
                //                 log_info!(
                //                     att_boolean_overload.logger(),
                //                     "command received - {:?}",
                //                     command
                //                 );
                //                 att_boolean_overload.set(command).await.unwrap();
                //             }
                //             .boxed()
                //         },
                //         None::<fn(&_) -> bool>,
                //     )
                //     .await;
            }
        }
    }

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Initialise et configure l'attribut booléen RW avec son callback
///
async fn create_rw_boolean_attribute(
    class: &mut impl panduza_platform_core::Container,
) -> Result<BooleanAttributeServer, Error> {
    //
    // Create the read-write boolean attribute
    let att_boolean_rw = class
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

    //
    // Set initial value
    att_boolean_rw.set(false).await?;

    //
    // Add a callback to handle commands for the read-write boolean attribute
    att_boolean_rw
        .add_callback({
            let att_boolean_rw = att_boolean_rw.clone();
            move |command| {
                let att_boolean_rw = att_boolean_rw.clone();
                async move {
                    log_info!(
                        att_boolean_rw.logger(),
                        "command received - {:?}",
                        command.try_value()
                    );
                    att_boolean_rw.set(command).await.unwrap();
                }
                .boxed()
            }
        })
        .await;

    //
    // Return the read-write boolean attribute server
    Ok(att_boolean_rw)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Initialise et configure l'attribut booléen WO pour la simulation d'alerte avec son callback
async fn create_alert_boolean_attribute(
    class: &mut impl panduza_platform_core::Container,
) -> Result<BooleanAttributeServer, Error> {
    //
    // Create the write-only boolean attribute for alert simulation
    let att_boolean_alert = class
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
    // Add a callback to handle commands for the alert simulation attribute
    att_boolean_alert
        .add_callback({
            let att_boolean_alert = att_boolean_alert.clone();
            move |_command| {
                let att_boolean_alert = att_boolean_alert.clone();
                async move {
                    log_info!(att_boolean_alert.logger(), "Alert simulation triggered");
                    att_boolean_alert
                        .trigger_alert("Simulated alert triggered for testing purposes")
                        .await;
                }
                .boxed()
            }
        })
        .await;

    //
    // Return the write-only boolean attribute for alert simulation
    Ok(att_boolean_alert)
}
