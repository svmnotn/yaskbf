#![no_std]
#![no_main]

// use defmt::*;
use yaskbf::BLINK_PERIOD;
use embassy_executor::Spawner;
use embassy_nrf::{gpio::{Level, Output, OutputDrive}, interrupt::Priority};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // global logger
use nrf_softdevice as _;

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, embassy_nrf::peripherals::P0_15>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    let led = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    let _ = spawner.spawn(blinker(led, BLINK_PERIOD));
}
