#![no_std]
#![no_main]

use embassy_time::Duration;

pub const BLINK_PERIOD: Duration = Duration::from_secs(1);
