#![no_std]
#![no_main]

use esp_hal::{main, Config, delay::Delay};
use esp_backtrace as _;
/* To do: enable `log` logging. */
// use esp_println::{/*logger, dbg,*/ println};
// use log::{info, trace};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let _peripherals = esp_hal::init(Config::default());

    // To do: uncomment line.
    // logger::init_logger_from_env();

    let delay = Delay::new();
    /* ___try___: change `ESP_LOG` level to trace*/
    // trace!("My Trace Log");
    // info!("My Info Log");
    loop {
        // `loop` is `!` type, since it never returns.
        delay.delay_millis(2000);
        // println!("loop..!");
        /* ___try___: uncomment below */
        panic!("hi");
    }
}
