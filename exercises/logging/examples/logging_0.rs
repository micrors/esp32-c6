#![no_std]
#![no_main]
// esp_backtrace, esp_println, defmt
use esp_hal::{
    main,
    Config,
};
use esp_backtrace as _;
use esp_println::{logger,println};
use log::{info};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_hal::init(Config::default());
    logger::init_logger_from_env();
    info!("SOME INFO!");
    println!("println!");
    panic!("Welcome.")
}
