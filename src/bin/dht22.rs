#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use dht_sensor::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, OutputOpenDrain};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Pull the pin high for initialisation
    let mut sensor_pin = OutputOpenDrain::new(p.PIN_12, Level::High);

    // Wait for the sensor to initialise...
    info!("Waiting on the sensor...");
    Timer::after(Duration::from_secs(1)).await;

    // Loop
    loop {
        match dht22::read(&mut Delay, &mut sensor_pin).await {
            Ok(dht22::Reading {
                temperature,
                relative_humidity,
            }) => info!("{}°C, {}% RH", temperature, relative_humidity),
            Err(e) => match e {
                DhtError::PinError(_) => error!("Pin Error!"),
                DhtError::ChecksumMismatch => error!("Checksum Mismatch!"),
                DhtError::Timeout => error!("Timeout!"),
            },
        }

        // Wait 1s
        Timer::after(Duration::from_secs(1)).await;
    }
}
