#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]

use core::mem;
use defmt_rtt as _;
use embassy_executor::{main, task, Spawner};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::interrupt::InterruptExt;
use embassy_nrf::usb::Driver;
use embassy_nrf::{self as _, bind_interrupts}; // time driver
use embassy_time::Timer;
use nrf_softdevice::{
    ble::{
        advertisement_builder::{
            Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList,
            ServiceUuid16,
        },
        peripheral,
    },
    raw, Softdevice,
};
use panic_probe as _;
use static_cell::StaticCell;
use yaskbf::init_peripherals; // global logger

// #[task]
// async fn softdevice_task(sd: &'static Softdevice) -> ! {
//     sd.run().await
// }

#[task]
async fn heartbeat() -> ! {
    let p = init_peripherals();
    let mut led = Output::new(p.P0_15, Level::High, OutputDrive::Standard);
    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

pub fn get_usb_driver() -> embassy_usb::Builder<
    'static,
    Driver<'static, embassy_nrf::peripherals::USBD, impl embassy_nrf::usb::vbus_detect::VbusDetect>,
> {
    static VBUS_DETECT: once_cell::sync::OnceCell<
        embassy_nrf::usb::vbus_detect::SoftwareVbusDetect,
    > = once_cell::sync::OnceCell::new();

    bind_interrupts!(
        struct Irqs {
            USBD => embassy_nrf::usb::InterruptHandler<embassy_nrf::peripherals::USBD>;
            POWER_CLOCK => embassy_nrf::usb::vbus_detect::InterruptHandler;
        }
    );

    embassy_nrf::interrupt::USBD.set_priority(embassy_nrf::interrupt::Priority::P2);
    embassy_nrf::interrupt::POWER_CLOCK.set_priority(embassy_nrf::interrupt::Priority::P2);

    let mut config = embassy_usb::Config::new(0x4242, 0x6942);
    config.manufacturer.replace("svmnotn");
    config.product.replace("yaskbf");
    config.serial_number.replace("right-1");
    config.max_power = 100;

    let vbus_detect = VBUS_DETECT
        .get_or_init(|| embassy_nrf::usb::vbus_detect::SoftwareVbusDetect::new(true, true));

    let usb_driver = Driver::new(
        unsafe { embassy_nrf::peripherals::USBD::steal() },
        Irqs,
        vbus_detect,
    );

    static DEVICE_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let device_descriptor = DEVICE_DESCRIPTOR.init([0; 256]);
    static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let config_descriptor = CONFIG_DESCRIPTOR.init([0; 256]);
    static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let bos_descriptor = BOS_DESCRIPTOR.init([0; 256]);
    static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    let msos_descriptor = MSOS_DESCRIPTOR.init([0; 256]);
    static CONTROL_BUF: StaticCell<[u8; 128]> = StaticCell::new();
    let control_buf = CONTROL_BUF.init([0; 128]);

    embassy_usb::Builder::new(
        usb_driver,
        config,
        device_descriptor,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control_buf,
    )
}

#[main]
async fn main(spawner: Spawner) {
    let _ = spawner.spawn(heartbeat());

    let b = get_usb_driver();
    b.build().run().await;
    
    // info!()
    // let config = nrf_softdevice::Config {
    //     clock: Some(raw::nrf_clock_lf_cfg_t {
    //         source: raw::NRF_CLOCK_LF_SRC_RC as u8,
    //         rc_ctiv: 16,
    //         rc_temp_ctiv: 2,
    //         accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
    //     }),
    //     conn_gap: Some(raw::ble_gap_conn_cfg_t {
    //         conn_count: 6,
    //         event_length: 24,
    //     }),
    //     conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
    //     gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
    //         attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
    //     }),
    //     gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
    //         adv_set_count: 1,
    //         periph_role_count: 3,
    //         central_role_count: 3,
    //         central_sec_count: 0,
    //         _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
    //     }),
    //     gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
    //         p_value: b"HelloRust" as *const u8 as _,
    //         current_len: 9,
    //         max_len: 9,
    //         write_perm: unsafe { mem::zeroed() },
    //         _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
    //             raw::BLE_GATTS_VLOC_STACK as u8,
    //         ),
    //     }),
    //     ..Default::default()
    // };

    // let sd = Softdevice::enable(&config);
    // let _ = spawner.spawn(softdevice_task(sd));

    // let config = peripheral::Config { interval: 50, ..Default::default() };

    // static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
    //     .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
    //     .services_16(ServiceList::Complete, &[ServiceUuid16::HEALTH_THERMOMETER]) // if there were a lot of these there may not be room for the full name
    //     .short_name("Hello")
    //     .build();

    // // but we can put it in the scan data
    // // so the full name is visible once connected
    // static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
    //     .full_name("Hello, Rust!")
    //     .build();

    // let adv = peripheral::NonconnectableAdvertisement::ScannableUndirected {
    //     adv_data: &ADV_DATA,
    //     scan_data: &SCAN_DATA,
    // };
    // let _ = peripheral::advertise(sd, adv, &config).await;
}
