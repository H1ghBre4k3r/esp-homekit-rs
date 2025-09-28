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

use core::mem::MaybeUninit;
use core::pin::pin;

use alloc::boxed::Box;

use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_time::{Duration, Timer};

use esp_backtrace as _;
use esp_bootloader_esp_idf::partitions;
use esp_hal::timer::timg::TimerGroup;

use esp_storage::FlashStorage;
use log::info;

use rs_matter_embassy::epoch::epoch;
use rs_matter_embassy::matter::dm::clusters::desc::{self, ClusterHandler as _};
use rs_matter_embassy::matter::dm::clusters::on_off::{self, ClusterHandler as _};
use rs_matter_embassy::matter::dm::devices::test::{TEST_DEV_ATT, TEST_DEV_COMM, TEST_DEV_DET};
use rs_matter_embassy::matter::dm::devices::DEV_TYPE_ON_OFF_LIGHT;
use rs_matter_embassy::matter::dm::{Async, Dataver, EmptyHandler, Endpoint, EpClMatcher, Node};
use rs_matter_embassy::matter::utils::init::InitMaybeUninit;
use rs_matter_embassy::matter::utils::select::Coalesce;
use rs_matter_embassy::matter::{clusters, devices};
use rs_matter_embassy::rand::esp::{esp_init_rand, esp_rand};
use rs_matter_embassy::stack::persist::{DummyKvBlobStore, KvBlobStore};
use rs_matter_embassy::wireless::esp::EspWifiDriver;
use rs_matter_embassy::wireless::{EmbassyWifi, EmbassyWifiMatterStack};

use embedded_storage::{ReadStorage, Storage};

extern crate alloc;

const BUMP_SIZE: usize = 15500;

esp_bootloader_esp_idf::esp_app_desc!();

struct MyStorage {
    flash: FlashStorage,
}

impl KvBlobStore for MyStorage {
    async fn load<F>(
        &mut self,
        key: u16,
        buf: &mut [u8],
        cb: F,
    ) -> Result<(), rs_matter_embassy::matter::error::Error>
    where
        F: FnOnce(Option<&[u8]>) -> Result<(), rs_matter_embassy::matter::error::Error>,
    {
        info!("Load: {key} {buf:?}");
        if let Err(e) = self.flash.read(key as u32, buf) {
            panic!("{e:?}")
        }

        if let Err(e) = cb(Some(buf)) {
            log::error!("{e:?}")
        }

        Ok(())
    }

    async fn store<F>(
        &mut self,
        key: u16,
        buf: &mut [u8],
        cb: F,
    ) -> Result<(), rs_matter_embassy::matter::error::Error>
    where
        F: FnOnce(&mut [u8]) -> Result<usize, rs_matter_embassy::matter::error::Error>,
    {
        info!("Store: {key} {buf:?}");
        if let Err(e) = self.flash.write(key as u32, buf) {
            panic!("{e:?}")
        }

        if let Err(e) = cb(buf) {
            log::error!("{e:?}")
        }

        Ok(())
    }

    async fn remove(
        &mut self,
        key: u16,
        buf: &mut [u8],
    ) -> Result<(), rs_matter_embassy::matter::error::Error> {
        todo!()
    }
}

#[esp_hal_embassy::main]
async fn main(_s: Spawner) {
    esp_println::logger::init_logger(log::LevelFilter::Info);

    info!("Starting...");

    // Heap strictly necessary only for Wifi+BLE and for the only Matter dependency which needs (~4KB) alloc - `x509`
    // However since `esp32` specifically has a disjoint heap which causes bss size troubles, it is easier
    // to allocate the statics once from heap as well
    init_heap();

    // == Step 1: ==
    // Necessary `esp-hal` and `esp-wifi` initialization boilerplate

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut flash = FlashStorage::new();
    info!("Flash size = {}", flash.capacity());
    let mut pt_mem = [0u8; partitions::PARTITION_TABLE_MAX_LEN];
    let pt = partitions::read_partition_table(&mut flash, &mut pt_mem).unwrap();

    for i in 0..pt.len() {
        let raw = pt.get_partition(i).unwrap();
        info!("{:?}", raw);
    }

    // The app descriptor (if present) is contained in the first 256 bytes
    // of an app image, right after the image header (24 bytes) and the first
    // section header (8 bytes)
    let mut app_desc = [0u8; 256];
    pt.find_partition(partitions::PartitionType::App(
        partitions::AppPartitionSubType::Factory,
    ))
    .unwrap()
    .unwrap()
    .as_embedded_storage(&mut flash)
    .read(32, &mut app_desc)
    .unwrap();
    info!("App descriptor dump {:02x?}\n", app_desc);

    let nvs = pt
        .find_partition(partitions::PartitionType::Data(
            partitions::DataPartitionSubType::Nvs,
        ))
        .unwrap()
        .unwrap();
    let mut nvs_partition = nvs.as_embedded_storage(&mut flash);

    let mut bytes = [0u8; 32];
    info!("NVS partition size = {}\n", nvs_partition.capacity());

    let offset_in_nvs_partition = 0;

    nvs_partition
        .read(offset_in_nvs_partition, &mut bytes)
        .unwrap();
    info!(
        "Read from {:x}:  {:02x?}\n",
        offset_in_nvs_partition,
        &bytes[..32]
    );
    let my_storage = MyStorage { flash };

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let rng = esp_hal::rng::Rng::new(peripherals.RNG);

    // To erase generics, `Matter` takes a rand `fn` rather than a trait or a closure,
    // so we need to initialize the global `rand` fn once
    esp_init_rand(rng);

    let init = esp_wifi::init(timg0.timer0, rng).unwrap();

    esp_hal_embassy::init(esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER).alarm0);

    // == Step 2: ==
    // Allocate the Matter stack.
    // For MCUs, it is best to allocate it statically, so as to avoid program stack blowups (its memory footprint is ~ 35 to 50KB).
    // It is also (currently) a mandatory requirement when the wireless stack variation is used.
    let stack =
        &*Box::leak(Box::new_uninit()).init_with(EmbassyWifiMatterStack::<BUMP_SIZE, ()>::init(
            &TEST_DEV_DET,
            TEST_DEV_COMM,
            &TEST_DEV_ATT,
            epoch,
            esp_rand,
        ));

    // == Step 3: ==
    // Our "light" on-off cluster.
    // Can be anything implementing `rs_matter::data_model::AsyncHandler`
    let on_off = on_off::OnOffHandler::new(Dataver::new_rand(stack.matter().rand()));

    // Chain our endpoint clusters
    let handler = EmptyHandler
        // Our on-off cluster, on Endpoint 1
        .chain(
            EpClMatcher::new(
                Some(LIGHT_ENDPOINT_ID),
                Some(on_off::OnOffHandler::CLUSTER.id),
            ),
            Async(on_off::HandlerAdaptor(&on_off)),
        )
        // Each Endpoint needs a Descriptor cluster too
        // Just use the one that `rs-matter` provides out of the box
        .chain(
            EpClMatcher::new(Some(LIGHT_ENDPOINT_ID), Some(desc::DescHandler::CLUSTER.id)),
            Async(desc::DescHandler::new(Dataver::new_rand(stack.matter().rand())).adapt()),
        );

    // == Step 4: ==
    // Run the Matter stack with our handler
    // Using `pin!` is completely optional, but saves some memory due to `rustc`
    // not being very intelligent w.r.t. stack usage in async functions
    //
    // This step can be repeated in that the stack can be stopped and started multiple times, as needed.
    let store = stack.create_shared_store(my_storage);
    let mut matter = pin!(stack.run_coex(
        // The Matter stack needs to instantiate an `embassy-net` `Driver` and `Controller`
        EmbassyWifi::new(
            EspWifiDriver::new(&init, peripherals.WIFI, peripherals.BT),
            stack
        ),
        // The Matter stack needs a persister to store its state
        &store,
        // Our `AsyncHandler` + `AsyncMetadata` impl
        (NODE, handler),
        // No user future to run
        (),
    ));

    // Just for demoing purposes:
    //
    // Run a sample loop that simulates state changes triggered by the HAL
    // Changes will be properly communicated to the Matter controllers
    // (i.e. Google Home, Alexa) and other Matter devices thanks to subscriptions
    let mut device = pin!(async {
        loop {
            // Simulate user toggling the light with a physical switch every 5 seconds
            Timer::after(Duration::from_secs(5)).await;

            // Toggle
            on_off.set(!on_off.get());

            // Let the Matter stack know that we have changed
            // the state of our Light device
            stack.notify_changed();

            info!("Light toggled");
        }
    });

    // Schedule the Matter run & the device loop together
    select(&mut matter, &mut device).coalesce().await.unwrap();
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
            device_types: devices!(DEV_TYPE_ON_OFF_LIGHT),
            clusters: clusters!(desc::DescHandler::CLUSTER, on_off::OnOffHandler::CLUSTER),
        },
    ],
};

#[allow(static_mut_refs)]
fn init_heap() {
    fn add_region<const N: usize>(region: &'static mut MaybeUninit<[u8; N]>) {
        unsafe {
            esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
                region.as_mut_ptr() as *mut u8,
                N,
                esp_alloc::MemoryCapability::Internal.into(),
            ));
        }
    }

    const HEAP_SIZE: usize = 186 * 1024;

    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    add_region(unsafe { &mut HEAP });
}
