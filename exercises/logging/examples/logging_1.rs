#![no_std]
#![no_main]

// Panic Handler.
use esp_backtrace as _;
// Needed as well, it's the Logger implemented.
// Used by the macros under the hood.
// use defmt::{debug, error, info, panic, println, trace, warn};
// use esp_println as _;

use esp_hal::{
    delay::Delay,
    time::{Duration, Instant},
    Config
};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    esp_hal::init(Config::default());

    // trace!("trace");
    // debug!("debug");
    // info!("info");
    // warn!("warn");
    // error!("error");

    let now = Instant::now();
    let delay = Delay::new();
    loop {
        // loop for 2 seconds, then exit and panic.
        println!("Loop...");
        delay.delay_millis(500u32);
        if now.elapsed() > Duration::from_millis(2000) {
            break;
        }
    }
    panic!("HAAAA!!!");
}
