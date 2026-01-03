#![no_std]
#![no_main]

use esp_hal::{
    // These imports are useful for the exercise
    gpio::{Level, Output, OutputConfig},
    time::{Duration, Instant},
};

use esp_backtrace as _;

esp_bootloader_esp_idf::esp_app_desc!();

const DELAY: u64 = 3000;
#[esp_hal::main]
fn main() -> ! {
    let config = esp_hal::Config::default();
    let peripherals = esp_hal::init(config);

    // Todo: Instantiate `OutputConfig` with default configuration
    /* let output_pin_config = ___fix_me___ */

    // Get GPIO7 pin from `peripherals`
    let gpio7 = peripherals.GPIO7;
    let level = Level::High;

    let mut led = Output::new(gpio7, level, output_pin_config);
    loop {
        // Todo: toggle `led`.
        /* led.fix_me */

        // Todo: The delay should be 3.5 seconds.
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(DELAY) {}
    }
}
