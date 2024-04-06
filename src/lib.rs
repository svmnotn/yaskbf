#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![macro_use]
pub use nrf_softdevice::__cortex_m_rt_SWI2_EGU2_trampoline;
pub use panic_probe::hard_fault;

mod display;
pub use display::display;
mod heartbeat;
pub use heartbeat::heartbeat;
mod joystick;
pub use joystick::joystick;
mod usb;
pub use usb::{usb, VBUS_DETECT};

pub fn init_peripherals() -> embassy_nrf::Peripherals {
    use embassy_nrf::{config::Config, interrupt::Priority};
    let mut config = Config::default();
    // config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    embassy_nrf::init(config)
}
