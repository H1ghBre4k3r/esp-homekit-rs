use embedded_storage::{ReadStorage, Storage};
use esp_bootloader_esp_idf::partitions::{self, FlashRegion, PartitionEntry};
use esp_storage::FlashStorage;
use log::info;
use rs_matter_embassy::stack::persist::KvBlobStore;

use crate::mk_static;

/// Simple key-value storage implementation using ESP32 NVS partition.
///
/// # Warning
///
/// This is a simplified NVS implementation with the following limitations:
/// - **No wear leveling**: Flash cells will wear out over time with repeated writes
/// - **Simple offset-based addressing**: Uses `(key + 1) * buffer_size` for data location
/// - **No garbage collection**: Removed data is marked invalid but space is not reclaimed
/// - **Limited key space**: Keys must be small enough that offsets fit in partition
/// - **No crash recovery**: Partial writes may leave inconsistent state
///
/// For production use, consider:
/// - Implementing proper wear leveling
/// - Using ESP-IDF's native NVS library (via `esp-idf-svc`)
/// - Adding checksums for data integrity
/// - Implementing atomic write-verify-commit sequences
pub struct Nvs {
    region: FlashRegion<'static, FlashStorage>,
}

impl Default for Nvs {
    fn default() -> Self {
        let flash = mk_static!(FlashStorage, FlashStorage::new());
        let pt_mem = mk_static!(
            [u8; partitions::PARTITION_TABLE_MAX_LEN],
            [0u8; partitions::PARTITION_TABLE_MAX_LEN]
        );
        let pt = partitions::read_partition_table(flash, pt_mem).unwrap();

        let nvs = mk_static!(
            PartitionEntry<'_>,
            pt.find_partition(partitions::PartitionType::Data(
                partitions::DataPartitionSubType::Nvs,
            ))
            .unwrap()
            .unwrap()
        );
        let region = nvs.as_embedded_storage(flash);
        info!("NVS partition size = {}\n", region.capacity());

        Self { region }
    }
}

impl Nvs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Nvs {
    async fn has(&mut self, key: u16) -> bool {
        let mut validity_flag = [0u8; 1];

        // Read the validity flag at the key's offset
        // 0 = data present, 255 = data removed/not present
        if let Err(e) = self.region.read(key as u32, &mut validity_flag) {
            log::error!("Failed to check existence of key {}: {:?}", key, e);
            return false;
        }

        validity_flag[0] == 0
    }
}

impl KvBlobStore for Nvs {
    async fn load<F>(
        &mut self,
        key: u16,
        buf: &mut [u8],
        cb: F,
    ) -> Result<(), rs_matter_embassy::matter::error::Error>
    where
        F: FnOnce(Option<&[u8]>) -> Result<(), rs_matter_embassy::matter::error::Error>,
    {
        let index = (key as usize + 1) * buf.len();
        info!("LOADING: key={}, offset={}", key, index);

        // Check if data exists for this key
        if !self.has(key).await {
            // No data for this key, invoke callback with None
            return cb(None);
        }

        // Read the data from flash
        if let Err(e) = self.region.read(index as u32, buf) {
            log::error!("Failed to read data for key {} at offset {}: {:?}", key, index, e);
            return cb(None);
        }

        // Invoke callback with the data
        cb(Some(buf))
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
        let index = (key as usize + 1) * buf.len();
        info!("STORING: key={}, offset={}", key, index);

        // Let the callback fill the buffer with data to store
        let _len = cb(buf)?;

        // Write the data to flash
        if let Err(e) = self.region.write(index as u32, buf) {
            log::error!("Failed to write data for key {} at offset {}: {:?}", key, index, e);
            return Err(rs_matter_embassy::matter::error::Error::new(
                rs_matter_embassy::matter::error::ErrorCode::NoSpace
            ));
        }

        // Mark the data as valid by writing 0 to the validity flag
        if let Err(e) = self.region.write(key as u32, &[0]) {
            log::error!("Failed to write validity flag for key {}: {:?}", key, e);
            return Err(rs_matter_embassy::matter::error::Error::new(
                rs_matter_embassy::matter::error::ErrorCode::NoSpace
            ));
        }

        Ok(())
    }

    async fn remove(
        &mut self,
        key: u16,
        _buf: &mut [u8],
    ) -> Result<(), rs_matter_embassy::matter::error::Error> {
        // Mark data as removed by writing 255 to the validity flag
        if let Err(e) = self.region.write(key as u32, &[255]) {
            log::error!("Failed to remove key {}: {:?}", key, e);
            return Err(rs_matter_embassy::matter::error::Error::new(
                rs_matter_embassy::matter::error::ErrorCode::NoSpace
            ));
        }

        info!("Removed key={}", key);
        Ok(())
    }
}
