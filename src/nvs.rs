use embassy_embedded_hal::adapter::BlockingAsync;
use esp_storage::FlashStorage;
use log::info;
use rs_matter_embassy::{persist::EmbassyKvBlobStore, stack::persist::KvBlobStore};

pub fn get_persistent_store() -> impl KvBlobStore {
    use esp_bootloader_esp_idf::partitions::{
        read_partition_table, DataPartitionSubType, PartitionType, PARTITION_TABLE_MAX_LEN,
    };

    let mut flash = FlashStorage::new();
    let mut pt_mem = [0u8; PARTITION_TABLE_MAX_LEN];
    let pt = read_partition_table(&mut flash, &mut pt_mem).unwrap();
    let nvs = pt
        .find_partition(PartitionType::Data(DataPartitionSubType::Nvs))
        .unwrap()
        .unwrap();

    let start = nvs.offset();
    let end = nvs.offset() + nvs.len();
    info!("Found persistent partition at {:#x}..{:#x}", start, end);

    EmbassyKvBlobStore::new(
        BlockingAsync::new(flash),
        // 0x9000..0xf000,
        start..end,
    )
}
