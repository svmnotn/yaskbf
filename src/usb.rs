use defmt::info;
use embassy_executor::task;
use embassy_nrf::{
    bind_interrupts,
    interrupt::{self, InterruptExt, Priority},
    peripherals,
    usb::{
        self,
        vbus_detect::{SoftwareVbusDetect, VbusDetect},
        Driver,
    },
};
use embassy_usb::{Builder, Config};
use once_cell::sync::OnceCell;
use static_cell::StaticCell;

pub static VBUS_DETECT: OnceCell<SoftwareVbusDetect> = OnceCell::new();

fn get_usb_driver(
) -> embassy_usb::Builder<'static, Driver<'static, peripherals::USBD, impl VbusDetect>> {
    info!("setup usb driver");
    let usb_driver = {
        unsafe {
            nrf_softdevice::raw::sd_power_usbpwrrdy_enable(true as u8);
            nrf_softdevice::raw::sd_power_usbdetected_enable(true as u8);
            nrf_softdevice::raw::sd_power_usbremoved_enable(true as u8);
        }

        bind_interrupts!(
            struct Irqs {
                USBD => usb::InterruptHandler<peripherals::USBD>;
            }
        );

        interrupt::USBD.set_priority(Priority::P2);

        let vbus_detect = VBUS_DETECT.get_or_init(|| SoftwareVbusDetect::new(true, true));

        Driver::new(unsafe { peripherals::USBD::steal() }, Irqs, vbus_detect)
    };

    let mut config = Config::new(0x4242, 0x6942);
    config.manufacturer.replace("svmnotn");
    config.product.replace("yaskbf");
    config.serial_number.replace("right-1");
    config.max_power = 100;

    static DEVICE_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let device_descriptor = DEVICE_DESCRIPTOR.init([0; 256]);
    static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let config_descriptor = CONFIG_DESCRIPTOR.init([0; 256]);
    static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let bos_descriptor = BOS_DESCRIPTOR.init([0; 256]);
    static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let msos_descriptor = MSOS_DESCRIPTOR.init([0; 256]);

    Builder::new(
        usb_driver,
        config,
        device_descriptor,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
    )
}

#[task]
pub async fn usb() -> ! {
    let b = get_usb_driver();
    let mut b = b.build();
    b.run().await
}
