// View output with `screen /dev/ttyACM0 115200`
// Use "reset" once after opening it.
#![no_std]
#![no_main]

use esp_backtrace as _;
// use esp_println as _;
use esp_hal::{
    Blocking, Config, main,
    uart::{self, Parity, Uart},
};

// make app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

// Constants
const WELCOME_MSG: &[u8] = b"Welcome to UART echo server.\r\n";
const MAX_LEN: usize = /*___fix_me__*/;
const CR: u8 = 0x0D; // Carriage Return hex code.
const LF: u8 = 0x0A; // Line Feed hex code.

#[main]
fn main() -> ! {
    // Take inspiration from previous exercises ;)
    let peripherals = /*___fix_me___*/

    /* ___fix_me___: configure params other that _parity.*/
    let uart_config = uart::Config::default()
        .with_parity(Parity::None)

    // Wrap peripheral in a driver, along with the config.
    let mut uart = Uart::new(/*___fix_me___*/)
        .expect("Should set up the driver.")
        // The UART periph will use these pins for comms.
        // These aren't pin-headers, though they may
        // connect to some pin headers.
        .with_tx(peripherals.GPIO16)
        .with_rx(peripherals.GPIO17);

    // Uart implements Write
    // It sends data through GPIO16.
    write_flush(&mut uart, WELCOME_MSG);
    info!("Flushed message, yay!");

    // Read the code until it's understood,
    // And feel free to improve it or play with it.
    let mut buf = [0u8; MAX_LEN];
    let mut index = 0;
    while index < MAX_LEN - 1 {
        let bytes_read =  uart.read(&mut buf[index..])
            .expect("Should read data from: laptop=>Uart(Rx).");
        index += bytes_read;
        if index == 1 {
            let last = buf.first().unwrap();
            if *last == CR || *last == LF {
                buf[0] = CR;
                buf[1] = LF;
                write_flush_reset(&mut uart,&mut buf,&mut index);
            }
        } else if index > 1 {
            match (buf[index - 2], buf[index - 1]) {
                (CR, LF) => (),
                (_, LF) => {
                    buf[index - 1] = CR;
                    buf[index] = LF;
                    index+=1
                }
                (_, CR) => {
                    buf[index] = LF;
                    index+=1
                }
                _ => continue,
            };
            write_flush(&mut uart, b"You said: ");
            write_flush_reset(&mut uart, &mut buf[..index], &mut index);
        }
    }
    panic!("Too much talk.")
}

/// MCU(Tx) => Laptop(Rx)
fn write_flush(uart: &mut Uart<'_, Blocking>, buf: &[u8]) {
    uart.write(buf).expect("Should write bytes to buffer.");
    uart.flush().expect("Should send bytes from UART_Tx to laptop.");
}

/// Write to buffer, flush to laptop, reset state.
fn write_flush_reset(uart:&mut Uart<'_, Blocking>, buf:&mut [u8], index:&mut usize){
    // By having these together we avoid the bug of
    // forgetting to reset after flushing.
    write_flush(uart, buf);
    // Reset state
    // overwrite what we had written with 0s.
    buf[..*index].fill(0);
    // reset index
    *index = 0;
}

