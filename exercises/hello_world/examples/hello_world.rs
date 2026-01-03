#![no_std]
#![no_main]

use esp_hal::{main, Config, delay::Delay};
// Handles our code panicking, and if we use `panic!`.
use esp_backtrace as _;

// Loggers
// We also have `print` (does not add a new line).
use esp_println::{logger, dbg, println};
use log::{info, trace};

// More on: https://docs.espressif.com/projects/rust/esp-bootloader-esp-idf/0.4.0/esp32/esp_bootloader_esp_idf/index.html
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Initialise the peripherals with default config.
    let _peripherals = esp_hal::init(Config::default());
    dbg!("Hi from dbg!");

    // Loggers must be initialised
    // then they provide `info!` etc.
    logger::init_logger_from_env();

    let delay = Delay::new();
    // Try changing the ESP_LOG level to trace
    trace!("My Trace Log");
    info!("My Info Log");
    loop {
        // `loop` is `!` type, since it never returns.
        // We are good!
        delay.delay_millis(2000);
        println!("loop..!");
    }
}
