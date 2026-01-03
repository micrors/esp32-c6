#![no_std]
#![no_main]

// entry point attribute
use esp_hal::main;

// a panic handler is always needed,
// even if we dont use panic!
use esp_backtrace as _;

// We also need a log::Logger implementation
// Which is what esp_println provides.
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // We always need to initialise the peripherals.
    esp_hal::init(esp_hal::Config::default());

    println!("Hello world!");

    panic!("This is a panic");
}
