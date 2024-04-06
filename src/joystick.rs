use defmt::*;
use embassy_executor::task;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Input},
    saadc::{self, AnyInput},
    PeripheralRef,
};
use embassy_time::Timer;

#[task]
pub async fn joystick(
    saadc: PeripheralRef<'static, embassy_nrf::peripherals::SAADC>,
    x: AnyInput,
    y: AnyInput,
    pressed: AnyPin,
) -> ! {
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    let config = saadc::Config::default();
    let chan_x = saadc::ChannelConfig::single_ended(x);
    let chan_y = saadc::ChannelConfig::single_ended(y);
    let mut saadc = saadc::Saadc::new(saadc, Irqs, config, [chan_x, chan_y]);
    let pressed = Input::new(pressed, embassy_nrf::gpio::Pull::Up);

    let mut avg_x = 0;
    let mut avg_y = 0;

    for _ in 0..500 {
        let mut buf = [0; 2];
        saadc.sample(&mut buf).await;
        saadc.calibrate().await;
        avg_x += buf[0] as i32;
        avg_y += buf[1] as i32;
        Timer::after_millis(1).await;
    }

    avg_x /= 500;
    avg_y /= 500;

    let offset_x = avg_x as i16;
    let offset_y = avg_y as i16;

    const SAMPLE_LENGTH: usize = 512;
    let mut samples_x = [0; SAMPLE_LENGTH];
    let mut samples_y = [0; SAMPLE_LENGTH];
    for i in 0..SAMPLE_LENGTH {
        let mut buf = [0; 2];
        saadc.sample(&mut buf).await;
        samples_x[i] = buf[0] - offset_x;
        samples_y[i] = buf[1] - offset_y;
        Timer::after_micros(10).await;
    }

    let mut oldest_sample = 0;
    loop {
        // disp.clear();
        let mut buf = [0; 2];
        saadc.sample(&mut buf).await;
        samples_x[oldest_sample] = buf[0] - offset_x;
        samples_y[oldest_sample] = buf[1] - offset_y;
        oldest_sample += 1;
        oldest_sample %= SAMPLE_LENGTH;

        let x =
            (samples_x.iter().map(|v| *v as isize).sum::<isize>() / SAMPLE_LENGTH as isize) as i16;
        let y =
            (samples_y.iter().map(|v| *v as isize).sum::<isize>() / SAMPLE_LENGTH as isize) as i16;
        if oldest_sample == 0 {
            saadc.calibrate().await;
            info!("x: {=i16}, y: {=i16}, btn: {=bool}", x, y, pressed.is_low());
        }
    }
}
