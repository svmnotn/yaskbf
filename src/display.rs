use defmt::info;
use embassy_executor::task;
use embassy_nrf::{
    self as _, bind_interrupts,
    gpio::{AnyPin, Level, Output, OutputDrive},
    spim::{self, Config as SpiConfig, Frequency},
    PeripheralRef,
};
use embassy_time::Timer;
use embedded_graphics_core::{geometry::OriginDimensions, pixelcolor::BinaryColor};
use sharp_memory_display::MemoryDisplay;

#[task]
pub async fn display(
    spi: PeripheralRef<'static, embassy_nrf::peripherals::SPI3>,
    sck: AnyPin,
    mosi: AnyPin,
    cs: AnyPin,
    disp: AnyPin,
) -> ! {
    bind_interrupts!(struct Irqs {
        SPIM3 => spim::InterruptHandler<embassy_nrf::peripherals::SPI3>;
    });
    let mut spi_config: SpiConfig = Default::default();
    spi_config.mode = sharp_memory_display::MODE;
    spi_config.frequency = Frequency::M8;
    spi_config.sck_drive = OutputDrive::Standard;
    spi_config.mosi_drive = OutputDrive::Standard;

    let spim = spim::Spim::new_txonly(spi, Irqs, sck, mosi, spi_config);
    let ncs = Output::new(cs, Level::Low, OutputDrive::Standard);
    let disp = Output::new(disp, Level::High, OutputDrive::Standard);
    let mut disp = MemoryDisplay::new(spim, ncs, disp);
    disp.enable();
    disp.clear();

    let size = disp.size();
    info!("Display has dims w: {} h: {}", size.width, size.height);

    let mut x_progress = 0;
    let mut y_progress = 1;
    loop {
        unsafe {
            for x in 0..x_progress {
                for y in 0..y_progress {
                    disp.set_pixel(x, y, BinaryColor::Off);
                }
            }
        }

        x_progress += 1;
        x_progress %= size.width;
        if x_progress == 0 {
            y_progress += 1;
            y_progress %= size.height;
            if y_progress == 0 {
                disp.clear();
            }
        }

        disp.flush_buffer();
        disp.display_mode();
        Timer::after_millis(10).await;
    }
}
