#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![macro_use]
pub use nrf_softdevice::__cortex_m_rt_SWI2_EGU2_trampoline;
pub use panic_probe::hard_fault;

pub const BLINK_PERIOD: embassy_time::Duration = embassy_time::Duration::from_millis(500);

use embassy_nrf::{config::Config, init, interrupt::Priority, Peripherals};
pub fn init_peripherals() -> Peripherals {
    let mut config = Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    init(config)
}
