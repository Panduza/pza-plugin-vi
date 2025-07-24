use futures::FutureExt;
use panduza_platform_core::instance::server::string::StringAttributeServer;
use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::Container;
use panduza_platform_core::Error;
use panduza_platform_core::Instance;

// Static constants for attribute info texts
const INFO_STRING_RO: &str = r#"# String Read Only Tester"#;
const INFO_STRING_WO: &str = r#"# String Write Only Tester"#;
const INFO_STRING_RW: &str = r#"# String Read Write Tester"#;

/// This module contains the implementation of the string attribute test.
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("string").finish().await;
    log_debug_mount_start!(class.logger());

    // Création des attributs de test string (ro, wo)
    create_string_test_attributes(&mut class).await?;

    //
    // Create a read-write string attribute
    create_rw_string_attribute(&mut class).await?;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Initialise et configure l'attribut string RW avec son callback
///
async fn create_rw_string_attribute(
    class: &mut impl panduza_platform_core::Container,
) -> Result<StringAttributeServer, Error> {
    //
    // Create the read-write string attribute
    let att_string_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(INFO_STRING_RW)
        .start_as_string()
        .await?;

    //
    // Set initial value
    att_string_rw.set("initial_value").await?;

    //
    // Add a callback to handle commands for the read-write string attribute
    att_string_rw
        .add_callback({
            let att_string_rw = att_string_rw.clone();
            move |command| {
                let att_string_rw = att_string_rw.clone();
                async move {
                    log_info!(
                        att_string_rw.logger(),
                        "command received - {:?}",
                        command.value()
                    );
                    att_string_rw
                        .set(command.value().unwrap())
                        .await
                        .expect("Failed to set value");
                }
                .boxed()
            }
        })
        .await;

    //
    // Return the read-write string attribute server
    Ok(att_string_rw)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Crée les attributs de test string (ro, wo) avec leurs callbacks
async fn create_string_test_attributes(
    class: &mut impl panduza_platform_core::Container,
) -> Result<(), Error> {
    // Création de l'attribut RO
    let att_string_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(INFO_STRING_RO)
        .start_as_string()
        .await?;
    att_string_ro.set("read_only_value").await?;

    // Attribut WO
    let att_string_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(INFO_STRING_WO)
        .start_as_string()
        .await?;

    att_string_wo
        .add_callback({
            let att_string_ro = att_string_ro.clone();
            move |command| {
                let att_string_ro = att_string_ro.clone();
                async move {
                    att_string_ro.set(command.value().unwrap()).await.unwrap();
                }
                .boxed()
            }
        })
        .await;

    Ok(())
}
