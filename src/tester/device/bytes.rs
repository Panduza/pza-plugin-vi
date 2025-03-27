use bytes::Bytes;
use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};
use std::sync::{Arc, Mutex};
use std::time::Instant;

//
//
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("bytes").finish().await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_bytes_ro = class
        .create_attribute("ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_bytes()
        .await?;

    //
    //
    att_bytes_ro.set(Bytes::from("initial")).await?;

    //
    // to stock the reception time of "ro"
    let mut timer_reception = class
        .create_attribute("counter_ro")
        .with_ro()
        .with_info(r#"read command"#)
        .start_as_number()
        .await?;

    timer_reception.set(0).await?;

    //
    // to reset the timer
    let mut timer_reset = class
        .create_attribute("counter_reset")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_boolean()
        .await?;

    timer_reset.set(true).await?;

    //
    //
    let mut att_bytes_wo = class
        .create_attribute("wo")
        .with_wo()
        .with_info(r#"write command"#)
        .start_as_bytes()
        .await?;

    // tokio::spawn(async move {
    //     loop {
    //         att_bytes_wo.wait_for_commands().await;
    //         while let Some(command) = att_bytes_wo.pop().await {
    //             log_info!(att_bytes_wo.logger(), "command recieved - {:?}", command);
    //             att_bytes_ro.set(command).await.unwrap();
    //         }
    //     }
    // });

    // timer shared between reception and reset
    let shared_timer = Arc::new(Mutex::new(None::<Instant>));

    let shared_timer_reset = shared_timer.clone();
    let timer_reception_reset = timer_reception.clone();

    // wait reset command and set timer to now and reception to 0
    tokio::spawn(async move {
        loop {
            timer_reset.wait_for_commands().await;
            while let Some(command) = timer_reset.pop().await {
                timer_reception_reset.set(0).await.unwrap();
                {
                    let mut timer = shared_timer_reset.lock().unwrap();
                    *timer = Some(Instant::now());
                }
            }
        }
    });

    //
    //
    tokio::spawn({
        let att_bytes_ro = att_bytes_ro.clone();
        let shared_timer = shared_timer.clone();
        let mut counter = 0;
        async move {
            loop {
                att_bytes_wo.wait_for_commands().await;
                while let Some(command) = att_bytes_wo.pop().await {
                    att_bytes_ro.set(command).await.unwrap();
                    if counter % 100 == 0 {
                        let elapsed = {
                            let timer = shared_timer.lock().unwrap();
                            timer.unwrap().elapsed().as_millis() as u32
                        };
                        timer_reception.set(elapsed).await.unwrap();
                    }
                    counter += 1;
                }
            }
        }
    });

    //
    //
    let mut att_bytes_rw = class
        .create_attribute("rw")
        .with_rw()
        .with_info(r#"read write command"#)
        .start_as_bytes()
        .await?;

    //
    //
    att_bytes_rw.set(Bytes::from("initial")).await?;

    //
    //
    tokio::spawn({
        let mut att_bytes_rw = att_bytes_rw.clone();
        async move {
            loop {
                att_bytes_rw.wait_for_commands().await;
                while let Some(command) = att_bytes_rw.pop().await {
                    log_info!(att_bytes_rw.logger(), "command received - {:?}", command);
                    att_bytes_rw.set(command).await.unwrap();
                }
            }
        }
    });

    // Finalize the mounting process
    log_debug_mount_end!(class.logger());
    Ok(())
}
