#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use core::mem;
use embassy_executor::{main, task, Spawner};
use embassy_nrf::{
    bind_interrupts,
    interrupt::{self, InterruptExt},
    gpio::{Input, Level, Output, OutputDrive, Pull},
    peripherals,
    usb::{self, vbus_detect::HardwareVbusDetect, Driver},
};
use embassy_time::Timer;
// use log::info;
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
use yaskbf::init_peripherals;
use defmt_rtt as _; // global logger

// #[task]
// async fn softdevice_task(sd: &'static Softdevice) -> ! {
//     sd.run().await
// }

// bind_interrupts!(struct Irqs {
//     USBD => usb::InterruptHandler<peripherals::USBD>;
//     POWER_CLOCK => usb::vbus_detect::InterruptHandler;
// });

#[main]
async fn main(spawner: Spawner) {
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
    // let spawn_value = spawner.spawn(softdevice_task(sd));

    let p = init_peripherals();
    // let usb = p.USBD;
    // interrupt::USBD.set_priority(interrupt::Priority::P2);
    // let driver = Driver::new(usb, Irqs, HardwareVbusDetect::new(Irqs));
    // embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);

    // info!("Init!");
    // info!("Init! {:?}", spawn_value);

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
    // info!("{:?}", peripheral::advertise(sd, adv, &config).await);

    let mut led = Output::new(p.P0_15, Level::High, OutputDrive::Standard);
    // let mut key = Input::new(p.P0_08, Pull::Up);
    loop {
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
        // key.wait_for_any_edge().await;
        // let lvl = key.get_level();
        // // info!("P0_08: {:?}", lvl);
        // match lvl {
        //     Level::Low => led.set_low(),
        //     Level::High => led.set_high(),
        // }
    }
}
