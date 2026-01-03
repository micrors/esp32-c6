#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{Config, delay::Delay};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(Config::default());
    let mut a = 2;
    a += 1;
    a -= 2;
    a *= 2;
    a /= 2;
    let delay = Delay::new();
    loop {
        delay.delay_millis(2000);
        println!("Loop!");
    }
}
