#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_rp::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

static BLINK_FREQ: AtomicU32 = AtomicU32::new(1);
static FREQ_CHANGED: Signal<()> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    // Poweroff the onboard LED
    Output::new(p.PIN_25, Level::Low);

    // Set up the other two LEDs (one on, one off)
    let leds = [
        Output::new(p.PIN_13.degrade(), Level::High),
        Output::new(p.PIN_15.degrade(), Level::Low),
    ];

    // Set up a pull-down button
    let mut button = Input::new(p.PIN_14, Pull::Down);

    // Spawn the blinker task
    unwrap!(spawner.spawn(blinker(leds)));

    // Loop
    loop {
        // Wait for a the button to be pressed...
        button.wait_for_falling_edge().await;

        // Update blinker frequency (double and wrap after 64 Hz)
        let new_freq = BLINK_FREQ.load(Ordering::SeqCst) * 2 % 127;
        BLINK_FREQ.store(new_freq, Ordering::SeqCst);
        info!("Updated frequency: {} Hz", new_freq);

        // Signal the the blinker to update frequency immediately, skipping the remainder of it's
        // current delay and continuing the loop
        FREQ_CHANGED.signal(());
    }
}

#[embassy_executor::task]
async fn blinker(mut leds: [Output<'static, AnyPin>; 2]) {
    loop {
        // Swap LED states
        for led in &mut leds {
            led.toggle();
        }

        // Wait an amount of time or for the frequency to be updated
        let delay = 1000 / BLINK_FREQ.load(Ordering::SeqCst);
        select(
            Timer::after(Duration::from_millis(delay.into())),
            FREQ_CHANGED.wait(),
        )
        .await;
    }
}
