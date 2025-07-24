use bytes::Bytes;
use futures::FutureExt;
use panduza_platform_core::instance::server::bytes::BytesAttributeServer;
use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::Container;
use panduza_platform_core::Error;
use panduza_platform_core::Instance;

// Static constants for attribute info texts
const INFO_BYTES_RO: &str = r#"# Bytes Read Only Tester"#;
const INFO_BYTES_WO: &str = r#"# Bytes Write Only Tester"#;
const INFO_BYTES_RW: &str = r#"# Bytes Read Write Tester"#;

/// This module contains the implementation of the bytes attribute test.
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("bytes").finish().await;
    log_debug_mount_start!(class.logger());

    // Création des attributs de test bytes (ro, wo)
    create_bytes_test_attributes(&mut class).await?;

    //
    // Create a read-write bytes attribute
    create_rw_bytes_attribute(&mut class).await?;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Initialise et configure l'attribut bytes RW avec son callback
///
async fn create_rw_bytes_attribute(
    class: &mut impl panduza_platform_core::Container,
) -> Result<BytesAttributeServer, Error> {
    //
    // Create the read-write bytes attribute
    let att_bytes_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(INFO_BYTES_RW)
        .start_as_bytes()
        .await?;

    //
    // Set initial value
    att_bytes_rw.set(Bytes::new()).await?;

    //
    // Add a callback to handle commands for the read-write bytes attribute
    att_bytes_rw
        .add_callback({
            let att_bytes_rw = att_bytes_rw.clone();
            move |command| {
                let att_bytes_rw = att_bytes_rw.clone();
                async move {
                    log_info!(
                        att_bytes_rw.logger(),
                        "command received - {:?}",
                        command.value()
                    );
                    att_bytes_rw.set(command.value().unwrap()).await;
                }
                .boxed()
            }
        })
        .await;

    //
    // Return the read-write bytes attribute server
    Ok(att_bytes_rw)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Crée les attributs de test bytes (ro, wo) avec leurs callbacks
async fn create_bytes_test_attributes(
    class: &mut impl panduza_platform_core::Container,
) -> Result<(), Error> {
    // Création de l'attribut RO
    let att_bytes_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(INFO_BYTES_RO)
        .start_as_bytes()
        .await?;
    att_bytes_ro.set(Bytes::new()).await?;

    // Attribut WO
    let att_bytes_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(INFO_BYTES_WO)
        .start_as_bytes()
        .await?;

    att_bytes_wo
        .add_callback({
            let att_bytes_ro = att_bytes_ro.clone();
            move |command| {
                let att_bytes_ro = att_bytes_ro.clone();
                async move {
                    att_bytes_ro.set(command.value().unwrap()).await.unwrap();
                }
                .boxed()
            }
        })
        .await;

    Ok(())
}
