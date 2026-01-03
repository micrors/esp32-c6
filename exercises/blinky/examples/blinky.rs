#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    time::{Duration, Instant},
};
esp_bootloader_esp_idf::esp_app_desc!();

// The type is given in the ::from_millis()
// parameters.
const DELAY: u64 = 500;

#[esp_hal::main]
fn main() -> ! {
    // Get peripherals
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    // Configure our OutputPin driver
    // 1. Select pin
    let gpio7 = peripherals.GPIO7;
    // 2. Set initial level
    let level = Level::High;
    // 3. Set driver mode and other config.
    let output_pin_config = OutputConfig::default();
    let mut led = Output::new(gpio7, level, output_pin_config);

    loop {
        led.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(DELAY) {}
    }
}
