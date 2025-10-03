//! An example utilizing the `EmbassyWifiMatterStack` struct.
//!
//! As the name suggests, this Matter stack assembly uses Wifi as the main transport,
//! and thus BLE for commissioning.
//!
//! If you want to use Ethernet, utilize `EmbassyEthMatterStack` instead.
//! If you want to use non-concurrent commissioning, call `run` instead of `run_coex`
//! and provision a higher `BUMP_SIZE` because the non-concurrent commissioning currently has a much-higher
//! memory requirements on the futures' sizes.
//! (Note: Alexa does not work (yet) with non-concurrent commissioning.)
//!
//! The example implements a fictitious Light device (an On-Off Matter cluster).
#![no_std]
#![no_main]
#![recursion_limit = "256"]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::rmt::{ConstChannelAccess, Rmt, Tx};
use esp_hal::time::Rate;
use esp_hal::timer::timg::{TimerGroup, Wdt};

use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use esp_homekit::credentials::credentials;
use esp_homekit::nvs::get_persistent_store;
use esp_homekit::util::blink;
use esp_homekit::{
    color_control, identify, level_control, mk_static, LightController, LED_COUNT, LED_SIZE,
};
use log::info;

use rs_matter::dm::clusters::basic_info::BasicInfoConfig;
use rs_matter::dm::devices::test::{TEST_PID, TEST_VID};
use rs_matter::dm::DeviceType;
use rs_matter_embassy::epoch::epoch;
use rs_matter_embassy::matter::dm::clusters::desc::{self, ClusterHandler as _};
use rs_matter_embassy::matter::dm::clusters::on_off::{self};
use rs_matter_embassy::matter::dm::devices::test::TEST_DEV_ATT;
use rs_matter_embassy::matter::dm::{Async, Dataver, EmptyHandler, Endpoint, EpClMatcher, Node};
use rs_matter_embassy::matter::{clusters, devices};
use rs_matter_embassy::rand::esp::{esp_init_rand, esp_rand};
use rs_matter_embassy::wireless::esp::EspWifiDriver;
use rs_matter_embassy::wireless::{EmbassyWifi, EmbassyWifiMatterStack};
use smart_leds::{SmartLedsWrite as _, RGB8, RGBA};

extern crate alloc;

const BUMP_SIZE: usize = 20_000;

esp_bootloader_esp_idf::esp_app_desc!();

const HEAP_SIZE: usize = 186 * 1024;

const DEVICE_CONFIG: BasicInfoConfig = BasicInfoConfig {
    vid: TEST_VID,
    pid: TEST_PID,
    hw_ver: 1,
    hw_ver_str: "1",
    sw_ver: 1,
    sw_ver_str: "1",
    serial_no: "1234567890",
    product_name: "ESP32 Smart Light",
    vendor_name: "PescaDev",
    device_name: "ESP32 Smart Light",
    ..BasicInfoConfig::new()
};

#[embassy_executor::task]
async fn watchdog_feeder(wdt: &'static mut Wdt<esp_hal::peripherals::TIMG1<'static>>) {
    loop {
        Timer::after(Duration::from_secs(3)).await;
        info!("Feeding watchdog...");
        wdt.feed();
    }
}

#[esp_hal_embassy::main]
async fn main(s: Spawner) {
    esp_println::logger::init_logger(log::LevelFilter::Info);

    info!("Starting...");

    // Heap strictly necessary only for Wifi+BLE and for the only Matter dependency which needs (~4KB) alloc - `x509`
    // However since `esp32` specifically has a disjoint heap which causes bss size troubles, it is easier
    // to allocate the statics once from heap as well
    heap_allocator!(size: HEAP_SIZE);

    // == Step 1: ==
    // Necessary `esp-hal` and `esp-wifi` initialization boilerplate
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut onboard: SmartLedsAdapter<ConstChannelAccess<Tx, 1>, 25> = SmartLedsAdapter::new(
        rmt.channel1,
        peripherals.GPIO8,
        smart_led_buffer!(LED_COUNT),
    );
    const LEVEL: u8 = 10;
    onboard
        .write([RGB8 {
            r: LEVEL,
            g: LEVEL,
            b: LEVEL,
        }])
        .unwrap();

    let led: SmartLedsAdapter<ConstChannelAccess<Tx, 0>, LED_SIZE, RGBA<u8>> =
        SmartLedsAdapter::new_with_color(
            rmt.channel0,
            peripherals.GPIO10,
            smart_led_buffer!(LED_COUNT; RGBW),
        );

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let rng = esp_hal::rng::Rng::new(peripherals.RNG);

    // To erase generics, `Matter` takes a rand `fn` rather than a trait or a closure,
    // so we need to initialize the global `rand` fn once
    esp_init_rand(rng);

    let init = esp_wifi::init(timg0.timer0, rng).unwrap();

    esp_hal_embassy::init(esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER).alarm0);

    // setup watchdog
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let mut wdt = timg1.wdt;
    wdt.set_timeout(
        esp_hal::timer::timg::MwdtStage::Stage0,
        esp_hal::time::Duration::from_secs(5),
    );
    wdt.enable();

    let wdt_static = mk_static!(Wdt<esp_hal::peripherals::TIMG1>, wdt);
    s.spawn(watchdog_feeder(wdt_static)).unwrap();

    Timer::after(Duration::from_millis(1000)).await;

    blink(&mut onboard, LEVEL, 0, 0).await;

    // Give BLE controller time to stabilize before Matter stack initialization
    // Fixes intermittent "BleHost(Hci(Invalid HCI Command Parameters))" errors
    Timer::after(Duration::from_millis(100)).await;

    // == Step 2: ==
    // Allocate the Matter stack.
    // For MCUs, it is best to allocate it statically, so as to avoid program stack blowups (its memory footprint is ~ 35 to 50KB).
    // It is also (currently) a mandatory requirement when the wireless stack variation is used.
    let stack = mk_static!(
        EmbassyWifiMatterStack<'_, BUMP_SIZE, ()>,
        EmbassyWifiMatterStack::<BUMP_SIZE, ()>::new(
            &DEVICE_CONFIG,
            credentials(),
            &TEST_DEV_ATT,
            epoch,
            esp_rand,
        )
    );

    blink(&mut onboard, 0, LEVEL, 0).await;

    let light_controller = mk_static!(
        LightController,
        LightController::new(Dataver::new_rand(stack.matter().rand()), led)
    );
    let light_control_handler = color_control::HandlerAdaptor(&*light_controller);
    let level_control_handler = level_control::HandlerAdaptor(&*light_controller);
    let on_off_handler = on_off::HandlerAdaptor(&*light_controller);
    let identify_handler = identify::HandlerAdaptor(&*light_controller);

    blink(&mut onboard, 0, LEVEL, 0).await;

    // Spawn transition task for smooth color/brightness changes
    s.spawn(esp_homekit::transition_task(light_controller))
        .unwrap();

    // == Step 3: ==
    // Our "light" on-off and color control clusters, both handled by LightController

    // Chain our endpoint clusters
    let handler = EmptyHandler
        // Our on-off cluster, on Endpoint 1
        .chain(
            EpClMatcher::new(
                Some(LIGHT_ENDPOINT_ID),
                Some(LightController::ON_OFF_CLUSTER.id),
            ),
            Async(on_off_handler),
        )
        // Identify cluster (device identification)
        .chain(
            EpClMatcher::new(
                Some(LIGHT_ENDPOINT_ID),
                Some(LightController::IDENTIFY_CLUSTER.id),
            ),
            Async(identify_handler),
        )
        // Level control cluster (brightness)
        .chain(
            EpClMatcher::new(
                Some(LIGHT_ENDPOINT_ID),
                Some(LightController::LEVEL_CONTROL_CLUSTER.id),
            ),
            Async(level_control_handler),
        )
        // Color control cluster
        .chain(
            EpClMatcher::new(
                Some(LIGHT_ENDPOINT_ID),
                Some(LightController::COLOR_CONTROL_CLUSTER.id),
            ),
            Async(light_control_handler),
        )
        // Each Endpoint needs a Descriptor cluster too
        // Just use the one that `rs-matter` provides out of the box
        .chain(
            EpClMatcher::new(Some(LIGHT_ENDPOINT_ID), Some(desc::DescHandler::CLUSTER.id)),
            Async(desc::DescHandler::new(Dataver::new_rand(stack.matter().rand())).adapt()),
        );

    blink(&mut onboard, 0, 0, LEVEL).await;

    // == Step 4: ==

    // Run the Matter stack with our handler
    // Using `pin!` is completely optional, but saves some memory due to `rustc`
    // not being very intelligent w.r.t. stack usage in async functions
    // This step can be repeated in that the stack can be stopped and started multiple times, as needed.
    // let store = stack.create_shared_store(Nvs::new());
    let store = stack.create_shared_store(get_persistent_store());
    blink(&mut onboard, 0, 0, LEVEL).await;
    stack
        .run_coex(
            // The Matter stack needs to instantiate an `embassy-net` `Driver` and `Controller`
            EmbassyWifi::new(
                EspWifiDriver::new(&init, peripherals.WIFI, peripherals.BT),
                stack,
            ),
            // The Matter stack needs a persister to store its state
            &store,
            // Our `AsyncHandler` + `AsyncMetadata` impl
            (NODE, handler),
            // No user future to run
            (),
        )
        .await
        .unwrap()
}

/// Endpoint 0 (the root endpoint) always runs
/// the hidden Matter system clusters, so we pick ID=1
const LIGHT_ENDPOINT_ID: u16 = 1;

/// The Matter Light device Node
const NODE: Node = Node {
    id: 0,
    endpoints: &[
        EmbassyWifiMatterStack::<0, ()>::root_endpoint(),
        Endpoint {
            id: LIGHT_ENDPOINT_ID,
            device_types: devices!(DeviceType {
                dtype: 0x010D,
                drev: 2
            }),
            clusters: clusters!(
                desc::DescHandler::CLUSTER,
                LightController::IDENTIFY_CLUSTER,
                LightController::ON_OFF_CLUSTER,
                LightController::LEVEL_CONTROL_CLUSTER,
                LightController::COLOR_CONTROL_CLUSTER
            ),
        },
    ],
};
