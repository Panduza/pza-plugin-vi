use std::time::Duration;

use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance
        .create_class("waveform")
        .with_tag("waveform")
        .finish()
        .await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_sample_ro = class
        .create_attribute("samples")
        .with_ro()
        .with_info(
            r#"---
        "#,
        )
        .start_as_vector_f32()
        .await?;
    att_sample_ro.set(&vec![0.0; 1]).await?;

    //
    //
    tokio::spawn(async move {
        let mut number_of_point = 500;
        loop {
            let step = 0.05;

            let mut data = Vec::new();
            for i in 0..number_of_point {
                data.push(f32::sin(i as f32 * step));
            }

            log_info!(
                att_sample_ro.logger(),
                "shoot {:?} ! {:?}bytes",
                number_of_point,
                number_of_point * size_of::<f32>()
            );

            att_sample_ro.set(&data).await.unwrap();

            tokio::time::sleep(Duration::from_secs(1)).await;

            number_of_point += 500;
            number_of_point %= 10000;
        }
    });

    log_debug_mount_end!(class.logger());
    Ok(())
}
