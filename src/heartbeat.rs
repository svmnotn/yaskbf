use embassy_executor::task;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive};
use embassy_time::{Duration, Timer};

pub const BLINK_PERIOD: Duration = Duration::from_millis(500);

#[task]
pub async fn heartbeat(led: AnyPin) -> ! {
    let mut led = Output::new(led, Level::High, OutputDrive::Standard);
    loop {
        led.toggle();
        Timer::after(BLINK_PERIOD).await;
    }
}
