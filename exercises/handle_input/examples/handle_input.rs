#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};

use esp_backtrace as _;
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    println!("Hello world!");

    // Todo: Configure GPIO7 as Output, with Level High
    // And default output config.
    let mut led = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());
    // Todo: Set GPIO9 as an Input. Use InputConfig to
    // configure it as pull up.
    // Check that BOOT button must be connected to GPIO9.
    // See the intro chapter's "user guide".
    let btn = Input::new(
        peripherals.GPIO9,
        InputConfig::default().with_pull(Pull::Up),
    );

    // Light the LED while the button is pressed,
    // And use a delay of 2seconds (so stays on 2s)
    // Turn off otherwise.
    let delay = Delay::new();
    loop {
        let is_pressed = btn.is_low();
        if is_pressed {
            println!("Pressed button.");
            led.set_high();
            delay.delay_millis(2000);
        } else {
            led.set_low();
        }
    }
}
