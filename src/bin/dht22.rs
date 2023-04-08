#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use dht_sensor::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Flex, Level, OutputOpenDrain, Pull};
use embassy_time::{Delay, Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Err, do I need this?
    let mut delay = Delay;

    // Pull the pin high for initialisation
    let mut sensor_pin = OutputOpenDrain::new(p.PIN_12, Level::High);
    // let mut sensor_pin = Flex::new(p.PIN_12);
    // sensor_pin.set_pull(Pull::None);
    // sensor_pin.set_as_output();
    // sensor_pin.set_high();

    loop {
        // Wait for the sensor to initialise...
        info!("Waiting on the sensor...");
        Timer::after(Duration::from_secs(1)).await;
        sensor_pin.set_low();
        Timer::after(Duration::from_millis(18)).await;
        sensor_pin.set_high();
    }

    // Loop
    loop {
        match dht22::Reading::read(&mut delay, &mut sensor_pin) {
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
