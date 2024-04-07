#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::{main, task, Spawner};
use embassy_nrf::{self as _, gpio::Pin, saadc::Input as SaadcInput, Peripheral};
use embassy_time::Timer;
use nrf_softdevice::Softdevice;
use panic_probe as _;
use yaskbf::{display, heartbeat, init_peripherals, joystick, usb, VBUS_DETECT};

#[task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    let vbus_detect = VBUS_DETECT
        .get_or_init(|| embassy_nrf::usb::vbus_detect::SoftwareVbusDetect::new(true, true));

    sd.run_with_callback(|e| match e {
        nrf_softdevice::SocEvent::PowerUsbPowerReady => {
            vbus_detect.ready();
        }
        nrf_softdevice::SocEvent::PowerUsbDetected => {
            vbus_detect.detected(true);
        }
        nrf_softdevice::SocEvent::PowerUsbRemoved => {
            vbus_detect.detected(false);
        }
        _ => {}
    })
    .await
}

#[main]
async fn main(spawner: Spawner) {
    let p = init_peripherals();
    unwrap!(spawner.spawn(heartbeat(p.P0_15.degrade())));
    unwrap!(spawner.spawn(joystick(
        p.SAADC.into_ref(),
        p.P0_29.degrade_saadc(),
        p.P0_31.degrade_saadc(),
        p.P1_15.degrade()
    )));
    // Wait for the joystick to properly initialize before starting the other tasks
    Timer::after_secs(1).await;
    unwrap!(spawner.spawn(display(
        p.SPI3.into_ref(),
        p.P1_02.degrade(),
        p.P1_01.degrade(),
        p.P1_07.degrade(),
        p.P0_09.degrade(),
    )));
    unwrap!(spawner.spawn(usb("right")));

    loop {
        Timer::after_secs(10).await;
        info!("Main Task");
    }
}
