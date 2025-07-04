use futures::FutureExt;
use panduza_platform_core::instance::server::number::NumberAttributeServer;
use panduza_platform_core::log_debug_mount_end;
use panduza_platform_core::log_debug_mount_start;
use panduza_platform_core::log_info;
use panduza_platform_core::Container;
use panduza_platform_core::Error;
use panduza_platform_core::Instance;

// Static constants for attribute info texts
const INFO_NUMBER_RO: &str = r#"# Number Read Only Tester"#;
const INFO_NUMBER_WO: &str = r#"# Number Write Only Tester"#;
const INFO_NUMBER_RW: &str = r#"# Number Read Write Tester"#;

/// This module contains the implementation of the number attribute test.
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("number").finish().await;
    log_debug_mount_start!(class.logger());

    // Création des attributs de test numériques (ro, wo_counter, wo)
    create_number_test_attributes(&mut class).await?;

    //
    // Create a read-write number attribute
    create_rw_number_attribute(&mut class).await?;

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Initialise et configure l'attribut numérique RW avec son callback
///
async fn create_rw_number_attribute(
    class: &mut impl panduza_platform_core::Container,
) -> Result<NumberAttributeServer, Error> {
    //
    // Create the read-write number attribute
    let att_number_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(INFO_NUMBER_RW)
        .start_as_number()
        .await?;

    //
    // Set initial value
    att_number_rw.set(0.0).await?;

    //
    // Add a callback to handle commands for the read-write number attribute
    att_number_rw
        .add_callback({
            let att_number_rw = att_number_rw.clone();
            move |command| {
                let att_number_rw = att_number_rw.clone();
                async move {
                    log_info!(
                        att_number_rw.logger(),
                        "command received - {:?}",
                        command.value()
                    );
                    att_number_rw.reply_to(&command, command.value()).await;
                }
                .boxed()
            }
        })
        .await;

    //
    // Return the read-write number attribute server
    Ok(att_number_rw)
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Crée les attributs de test numériques (ro, wo_counter, wo) avec leurs callbacks
async fn create_number_test_attributes(
    class: &mut impl panduza_platform_core::Container,
) -> Result<(), Error> {
    // Création de l'attribut RO
    let att_number_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(INFO_NUMBER_RO)
        .start_as_number()
        .await?;
    att_number_ro.set(0.0).await?;

    // Attribut WO
    let att_number_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(INFO_NUMBER_WO)
        .start_as_number()
        .await?;

    att_number_wo
        .add_callback({
            let att_number_ro = att_number_ro.clone();
            move |command| {
                let att_number_ro = att_number_ro.clone();
                async move {
                    att_number_ro.set(command.value()).await.unwrap();
                }
                .boxed()
            }
        })
        .await;

    Ok(())
}
