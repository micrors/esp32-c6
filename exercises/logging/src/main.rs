#![no_std]
#![no_main]
// esp_backtrace, esp_println, defmt
use esp_hal::{
    main,
    Config,
};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_hal::init(Config::default());
}
