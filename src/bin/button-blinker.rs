#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    peripherals::{PIN_13, PIN_15},
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

static BLINK_FREQ: AtomicU32 = AtomicU32::new(1);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Poweroff the onboard LED
    Output::new(p.PIN_25, Level::Low);

    // Set up the other two LEDs (one on, one off)
    // It would be nice to see if I could squish these into an array...
    let led_a = Output::new(p.PIN_13, Level::High);
    let led_b = Output::new(p.PIN_15, Level::Low);

    // Set up a pull-down button
    let mut button = Input::new(p.PIN_14, Pull::Down);

    // Spawn the blinker task
    unwrap!(spawner.spawn(blinker(led_a, led_b)));

    // Loop
    loop {
        // Wait for a the button to be pressed...
        button.wait_for_falling_edge().await;

        // Update blinker frequency
        let new_freq = BLINK_FREQ.load(Ordering::SeqCst) % 10 + 1;
        BLINK_FREQ.store(new_freq, Ordering::SeqCst);
        info!("Updated frequency: {}", new_freq);
    }
}

#[embassy_executor::task]
async fn blinker(mut led_a: Output<'static, PIN_13>, mut led_b: Output<'static, PIN_15>) {
    loop {
        // Swap LED states
        led_a.toggle();
        led_b.toggle();

        // Wait 1s
        let delay = 1000 / BLINK_FREQ.load(Ordering::SeqCst);
        Timer::after(Duration::from_millis(delay.into())).await;
    }
}
