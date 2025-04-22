use core::f32;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

use panduza_platform_core::{
    log_debug_mount_end, log_debug_mount_start, log_info, Container, Error, Instance,
};

///
///
pub async fn mount(mut instance: Instance) -> Result<(), Error> {
    //
    // Create interface
    let mut class = instance.create_class("vector_f32").finish().await;
    log_debug_mount_start!(class.logger());

    //
    //
    let att_sample_ro = class
        .create_attribute("ro")
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
        let amp = 2.0; // Amplitude
        let f = 0.1; // Frequency Hz
        let deltat = 20; // Sampling period ms
        let number_of_point = 1000;
        let mut last_x = 0;
        let mut rng = StdRng::from_entropy();

        loop {
            let step = deltat as f32 * 1.0e-3;

            let mut data = Vec::new();
            for i in last_x..(last_x + number_of_point) {
                let noise: f32 = rng.gen::<f32>() / 5.0;

                // log_info!(att_sample_ro.logger(), "throw {:?}", throw,);
                data.push(amp * f32::sin(i as f32 * 2.0 * f32::consts::PI * f * step) + noise);
            }

            log_info!(
                att_sample_ro.logger(),
                "shoot {:?} ! {:?}bytes",
                last_x,
                number_of_point * std::mem::size_of::<f32>()
            );

            att_sample_ro.set(&data).await.unwrap();

            last_x += number_of_point;
            tokio::time::sleep(Duration::from_millis(deltat)).await;
        }
    });

    log_debug_mount_end!(class.logger());
    Ok(())
}
