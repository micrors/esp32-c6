# Boilerplate

A few lines frequently repeat, like a mantra througout the examples. It helps a lot to analyse them.

## no_std, no_main

`no_std`, `no_main`, `no_std`, `no_main`, ...

The entry files start with

```rust
#![no_std]
#![no_main]
// ...
#[esp_hal::main]
fn main() -> ! { }
```

- `no_std` disables loading the `std` crate. It only loads `core` by default.
- `no_main` tells Rust not to search for a `main` function.
    - This is handled by `esp_hal::main` macro.

## Imports

A few imports also repeat througout:

```rust
use esp_hal::{main, Config};
use esp_backtrace as _;
use esp_println::println;
```

- `esp_hal`: provides `main` entry plus useful modules for dealing with peripherals, time, and so forth.
- `esp_backtrace`: Out of bounds indexing and our own `panic!` calls have to be handled by a panic-handler (otherwise provided by `std`).
    - This crate provides a replacement through the `panic-handler` feature.
    - In summary, it handles the `panic!` calls, and prints the backtrace (call stack up to that point.).
  <!-- Too much info. -->

  <!-- It prints via `esp_println`, added by the `println` feature. -->
  <!-- It can also print via `defmt` feature instead. -->

- `esp_println`: The macro `println!` comes [from `std`][println]. We don't use `std` so [`esp-println`][esp_println] provides the macro for us.
    - Logging backends (`log`, `defmt`) can also be used:
    ```diff
    use esp_println::{logger,println};
    +use log::{info, trace};
    ```
    - The `log` import isn't needed if we don't use these macros.

## App Descriptor

The line is:

```rust
esp_bootloader_esp_idf::esp_app_desc!();
```

- The board has 2 bootloaders.
    1. First stage bootloader: written in ROM and can't be changed. It's read first. It loads the second stage bootloader.
    2. The second stage bootloader needs an application descriptor, which `esp_bootloader_esp_idf::esp_app_desc!();` creates for us. This bootloader loads our application.
        > [!NOTE]
        > Each time we flash a binary, `espflash` includes a pre-compiled second stage bootloader with default settings, alongside our binary-code.

## init

The other line is within `fn main() { }` namely:

```rust
#[main]
fn main() -> ! {
    let _peripherals = esp_hal::init(Config::default());
}
```

This line initialises our MCU with default configuration. [In their words][esp_hal_docs_init], for `esp_hal::init`

> Initialize the system.
>
> This function sets up the CPU clock and watchdog, then, returns the peripherals and clocks.

Notice the "returns the peripherals". That's important. The `Peripherals` struct provides access to all of the hardware peripherals on the chip!

And for [`esp_hal::Config`][esp_hal_docs_config]:

> System configuration.
>
> This `struct` is marked with `#[non_exhaustive]` and can't be instantiated directly. This is done to prevent breaking changes when new fields are added to the `struct`. Instead, use the [`Config::default()`] method to create a new instance.

## Recap

So we have described the macro attributes (like `#[main]`), the module imports, the app descriptor and board initialisation.

> [!TIP]
> All the snippets in this page are so ubiquitous that it is useful to memorise them and their role.

## Suggested Reading

- [panic-handler].

[esp_hal_docs_config]: https://github.com/esp-rs/esp-hal/blob/9c7c809cec72b11db82d5f72f75d1c075ce654bb/esp-hal/src/lib.rs#L625
[esp_hal_docs_init]: https://github.com/esp-rs/esp-hal/blob/9c7c809cec72b11db82d5f72f75d1c075ce654bb/esp-hal/src/lib.rs#L728
[println]: https://doc.rust-lang.org/std/macro.println.html
[esp_println]: https://github.com/esp-rs/esp-hal/tree/main/esp-println
[panic-handler]: https://esp32.implrust.com/std-to-no-std/panic-handler.html

<!---->
<!-- # What is no_std? -->
<!---->
<!-- This tutorial is "`no_std`", but what does it mean? `std` can refer to either: -->
<!---->
<!-- - `std` the crate, or -->
<!-- - `std` the `rust-std` component, including the crates `std`, `core` and `alloc`. -->
<!---->
<!-- This book distinguishes these by adding the words *crate* and *component*, respectively. -->
<!---->
<!-- The microcontroller has no Operating System. Since `std` *crate* relies on one, `#[no_std]` is used in `main.rs` to exclude the `std` crate. -->
<!---->
<!-- To be precise, only `core` is included by default. A comparison is provided by this [the table][Rust Embedded Guide] below, copied from the Rust Embedded Guide: -->
<!---->
<!-- | feature                                                   | no\_std | std | -->
<!-- |-----------------------------------------------------------|---------|-----| -->
<!-- | heap (dynamic memory)                                     |    *    |  ✓  | -->
<!-- | collections (Vec, BTreeMap, etc)                          |   **    |  ✓  | -->
<!-- | stack overflow protection                                 |    ✘    |  ✓  | -->
<!-- | runs init code before main                                |    ✘    |  ✓  | -->
<!-- | libstd available                                          |    ✘    |  ✓  | -->
<!-- | libcore available                                         |    ✓    |  ✓  | -->
<!-- | writing firmware, kernel, or bootloader code              |    ✓    |  ✘  | -->
<!---->
<!-- \* Only if you use the `alloc` crate and use a suitable allocator like [esp-alloc]. -->
<!---->
<!-- \** Only if you use the `collections` crate and configure a global default allocator. -->
<!---->
<!-- \** HashMap and HashSet are not available due to a lack of a secure random number generator. -->
<!---->
<!---->
<!-- # What is no_std? -->
<!---->
<!-- This tutorial is "`no_std`", but what does it mean? `std` can refer to either: -->
<!---->
<!-- - `std` the crate, or -->
<!-- - `std` the `rust-std` component, including the crates `std`, `core` and `alloc`. -->
<!---->
<!-- This book distinguishes these by adding the words *crate* and *component*, respectively. -->
<!---->
<!-- The microcontroller has no Operating System. Since `std` *crate* relies on one, `#[no_std]` is used in `main.rs` to exclude the `std` crate. -->
<!---->
<!-- To be precise, only `core` is included by default. A comparison is provided by this [the table][Rust Embedded Guide] below, copied from the Rust Embedded Guide: -->
<!---->
<!-- | feature                                                   | no\_std | std | -->
<!-- |-----------------------------------------------------------|---------|-----| -->
<!-- | heap (dynamic memory)                                     |    *    |  ✓  | -->
<!-- | collections (Vec, BTreeMap, etc)                          |   **    |  ✓  | -->
<!-- | stack overflow protection                                 |    ✘    |  ✓  | -->
<!-- | runs init code before main                                |    ✘    |  ✓  | -->
<!-- | libstd available                                          |    ✘    |  ✓  | -->
<!-- | libcore available                                         |    ✓    |  ✓  | -->
<!-- | writing firmware, kernel, or bootloader code              |    ✓    |  ✘  | -->
<!---->
<!-- \* Only if you use the `alloc` crate and use a suitable allocator like [esp-alloc]. -->
<!---->
<!-- \** Only if you use the `collections` crate and configure a global default allocator. -->
<!---->
<!-- \** HashMap and HashSet are not available due to a lack of a secure random number generator.

[Rust Embedded Guide]: https://docs.rust-embedded.org/book/intro/no-std.html#overview

[esp-alloc]: https://docs.rs/crate/esp-alloc/latest
-->
