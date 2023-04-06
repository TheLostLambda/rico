#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Poweroff the onboard LED
    Output::new(p.PIN_25, Level::Low);

    // Set up the other two LEDs (one on, one off)
    let mut led_a = Output::new(p.PIN_13, Level::High);
    let mut led_b = Output::new(p.PIN_15, Level::Low);

    // Set up a pull-down button
    let button = Input::new(p.PIN_14, Pull::Down);

    // Loop
    loop {
        // Check if the button is pressed
        if button.is_low() {
            // Swap LED states
            led_a.toggle();
            led_b.toggle();

            // Inform the outside world of this exciting development!
            info!("Swap!");
        }

        // Wait 1s
        Timer::after(Duration::from_secs(1)).await;
    }
}
