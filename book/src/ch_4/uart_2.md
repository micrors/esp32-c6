# UART - Part 2

Consider our boilerplate:

```rust
#![no_std]
#![no_main]
use esp_hal::{main, Config};
use esp_backtrace as _;
use esp_println as _;
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
  let peripherals = esp_hal::init(Config::default())
  loop {}
}
```

How do we get to use a UART peripheral from there? How do we configure the peripheral? Which configuration options do you expect? Try getting a sense of what to do first (maybe checking `esp_hal` docs.)

>[!WARNING]
> The [pinouts] won't help. UART is connected to `UART-USB` bridge (a small IC near the USB ports). So when passing the `with_rx` and `with_tx` to the driver, we must choose the GPIOs hardwired to the UART-USB bridge. These are GPIO16, GPIO17. For sending to GPIO-header-pins (those on the sidelines) we could pass any GPIO (but not when using the bridge.)

<details>
<summary>======!Spoiler Alert!====== (click for a solution)</summary>

There are many other details in the `main.rs` file, but the most critical is to make sense of this snippet:

```rust
let uart_config = uart::Config::default()
    .with_baudrate(115200)
    // ... other settings

let mut uart = Uart::new(peripherals.UART0, uart_config)
    //   driver^^^^^^^^^       periph^^^^^ config^^^^^^
    .expect("Should set up the driver.")
    // Route through the UART-USB bridge
    .with_tx(peripherals.GPIO16)
    .with_rx(peripherals.GPIO17);
```

That is another boilerplate we can think of: `init` device, grab `peripheral`, pass it to a driver along with its config.

A list of some of the [peripherals] is:

```txt
# (Most of these are unstable)
GPIO0-23, GPIO27 peripheral singleton
I2C0 peripheral singleton
I2S0 peripheral singleton
LEDC peripheral singleton
RNG peripheral singleton
SPI0-2 peripheral singleton
TIMG0-1 peripheral singleton
UART0-1 peripheral singleton
USB_DEVICE peripheral singleton
WIFI peripheral singleton
```

</details>

## Exercise

1. Access `exercises/uart/src/main.rs`. The comments within will guide you through.
2. If not installed install `screen` (for example `sudo apt install screen`).
3. Before running it, note that we exit `screen` with `Ctrl+a+k` (then pressing `y` when prompted for it).
4. After flashing the board (any USB port) connect it through **ch343** (or **UART**) tagged port.
5. Run `screen <device_path> <baud_rate>`.
   - `<device_path>` is _probably_ `/dev/ttyACM0` on Linux and `/dev/tty.usbmodem0` MacOS.
   - To be sure, list them with `ls /dev/tty*` and try to figure out which one is the board.
   - `<baud_rate>` must match the rate in the code. For example `115200`.
6. Remember to press the RESET board button once.
7. Try the echo server by typing in the keyboard, and send with <kbd>Enter</kbd> key.

> [!NOTE]
> A solution is provided in `examples/uart.rs`.

## Imagining a projects

The esp32-c6 board has 2 UARTs: UART0 and UART1.

Technically, we could use UART0 just as we did, and UART1 could other GPIOs connected to GPIO-header-pins. By linking this to USB-UART converter and this in turn to a laptop, we could chat.

Two laptops could chat via the board.

## Suggested Reading

- This is a good time to check [`esp_hal`][esp_hal], especially their [peripherals] section, and anything you find interesting.

[esp_hal]: https://docs.espressif.com/projects/rust/esp-hal/1.0.0/esp32c6/esp_hal/index.html
[peripherals]: https://docs.espressif.com/projects/rust/esp-hal/1.0.0/esp32c6/esp_hal/peripherals/index.html
[pinouts]: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/_images/esp32-c6-devkitc-1-pin-layout.png
