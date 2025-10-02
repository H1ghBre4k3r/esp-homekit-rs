use esp_hal::efuse::Efuse;
use log::info;
use rs_matter::BasicCommData;

pub fn credentials() -> BasicCommData {
    // Read MAC address and derive commissioning credentials
    let mac = Efuse::read_base_mac_address();
    info!(
        "MAC: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    );

    // Derive 12-bit discriminator (0-4095) from MAC using XOR folding
    let discriminator: u16 =
        ((mac[0] ^ mac[2] ^ mac[4]) as u16 | ((mac[1] ^ mac[3] ^ mac[5]) as u16) << 8) & 0x0FFF;

    // Derive 32-bit password from MAC using byte combination + XOR mixing
    // Matter requires password in range 1-99,999,999 (0x5F5E0FF)
    let password_raw: u32 = u32::from_be_bytes([mac[0], mac[1], mac[2], mac[3]])
        ^ u32::from_be_bytes([mac[4], mac[5], mac[0], mac[1]]);
    let password: u32 = (password_raw % 99_999_999).max(1); // Constrain to valid range
    info!(
        "Commissioning: discriminator={}, password={}",
        discriminator, password
    );

    BasicCommData {
        password,
        discriminator,
    }
}
