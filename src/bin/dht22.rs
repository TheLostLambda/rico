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
        match dht22::Reading::read(&mut Delay, &mut sensor_pin) {
            Ok(dht22::Reading {
                temperature,
                relative_humidity,
            }) => info!("{}°, {}% RH", temperature, relative_humidity),
            Err(e) => match e {
                DhtError::PinError(_) => info!("Pin Error!"),
                DhtError::ChecksumMismatch => info!("Checksum Mismatch!"),
                DhtError::Timeout => info!("Timeout!"),
            },
        }

        // Wait 1s
        Timer::after(Duration::from_secs(1)).await;
    }
}
