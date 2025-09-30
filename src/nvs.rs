use embedded_storage::{ReadStorage, Storage};
use esp_bootloader_esp_idf::partitions::{self, FlashRegion, PartitionEntry};
use esp_storage::FlashStorage;
use log::info;
use rs_matter_embassy::stack::persist::KvBlobStore;

use crate::mk_static;

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
        let mut buf = [0u8; 32];

        if let Err(e) = self.region.read(0, &mut buf) {
            panic!("{e}");
        }

        buf[key as usize] == 0
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
        info!("LOADING: {index}");

        if !self.has(key).await {
            return cb(None);
        }

        if let Err(e) = self.region.read(index as u32, buf) {
            panic!("{e}");
        }

        if let Err(e) = cb(Some(buf)) {
            log::error!("{e}");
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
        let index = (key as usize + 1) * buf.len();
        info!("STORING: {index}");

        if let Err(e) = cb(buf) {
            log::error!("{e}");
        }

        if let Err(e) = self.region.write(index as u32, buf) {
            panic!("{e}");
        }

        if let Err(e) = self.region.write(key as u32, &[0]) {
            panic!("{e}");
        }

        Ok(())
    }

    async fn remove(
        &mut self,
        key: u16,
        _buf: &mut [u8],
    ) -> Result<(), rs_matter_embassy::matter::error::Error> {
        if let Err(e) = self.region.write(key as u32, &[255]) {
            panic!("{e}");
        }

        Ok(())
    }
}
