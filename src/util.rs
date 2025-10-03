use embassy_time::{Duration, Timer};
use esp_hal::rmt::{ConstChannelAccess, Tx};
use esp_hal_smartled::SmartLedsAdapter;
use smart_leds::{SmartLedsWrite as _, RGB8};

pub async fn blink(led: &mut SmartLedsAdapter<ConstChannelAccess<Tx, 1>, 25>, r: u8, g: u8, b: u8) {
    led.write([RGB8 { r, g, b }]).unwrap();
    Timer::after(Duration::from_millis(1000)).await;
    led.write([RGB8 { r: 0, g: 0, b: 0 }]).unwrap();
    Timer::after(Duration::from_millis(1000)).await;
}
