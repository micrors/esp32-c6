# Logging - Part 2

Dependencies that may log data provide support to use `defmt` or `log`. Previously, we configured a package to use `log`, so now it will be quicker, but other aspects can be examined.

As a recap:

- `espflash` has built in support for `defmt`, `log` or none using the `-L` flag.
- `esp-println` can use `defmt-espflash` or `log-04` features. As [their docs][esp-println] state:
    > Using the `defmt-espflash` feature, `esp-println` will install a `defmt` global logger.
- `esp-backtrace` can use `println` or `defmt` features, in order to print panic messages and backtraces.

Let's update the package to use `defmt`.

## Exercise

Go to `exercises/defmt` directory.

1. Update the runner's logger in `.cargo/config.toml`:

    ```diff
    [target.riscv32imac-unknown-none-elf]
    - runner = "espflash flash --monitor"
    + runner = "espflash flash --monitor -L defmt"
    ```

    so that `espflash` monitor can decode the format of the `defmt` messages received.

2. Update `Cargo.toml` to include the needed features:

    ```diff
    esp-println = { version = "0.16.0", features = [
        "esp32c6",
    -     "log-04",
    +     "defmt-espflash",
    ] }
    # (...)
    esp-backtrace = { version = "0.18.0", features = [
        "esp32c6",
        "panic-handler",
    -   "println"
    +   "defmt",
    ]}
    ```

3. Due to the [linking process] we need to add `defmt` linker script to `cargo/config.toml`:

    ```diff
    rustflags = [
      # ....
    +  "-C", "link-arg=-Tdefmt.x",
    ]
    ```

4. Add [defmt] to the dependencies, and remove `log`.
5. **Logging level**: Use the `defmt::println!` and some [defmt macros] to print a few messages.
   - When building the app, set [`DEFMT_LOG`][DEFMT_LOG] level as done for `ESP_LOG` earlier (within `.cargo/config.toml`, under `[env]` table).
   - An alternative to changing `.cargo/config.toml` is using `DEFMT_LOG=<value> cargo run --release`; the same is valid for `ESP_LOG`.
6. Add a `panic!` macro to trigger a panic with a `defmt` message.

>[!NOTE]
> `examples/logging_1.rs` contains a solution. You can run it with the following command: `cargo run --example logging_1 --release`. You will need to have the settings above done correctly though!

## Suggested reading

Short articles that give more context:

- defmt [linking process] for setting the compilation-time linker up.
- defmt [DEFMT_LOG] environment variable.
- esp-println [logging formats].

<!--esp links-->
[esp-println]: https://github.com/esp-rs/esp-hal/tree/main/esp-println
<!-- [esp-backtrace]: https://github.com/esp-rs/esp-hal/tree/main/esp-backtrace -->
<!-- [espflash]: https://github.com/esp-rs/espflash -->
[logging formats]: https://github.com/esp-rs/espflash/blob/main/espflash/README.md#logging-format

<!--defmt links-->
[defmt macros]: https://docs.rs/defmt/latest/defmt/#macros
[defmt]: https://docs.rs/defmt/latest/defmt/
[defmt docs]: http://defmt.ferrous-systems.com/introduction#operating-principle
[DEFMT_LOG]: https://defmt.ferrous-systems.com/filtering.html?highlight=DEFMT*LOG#defmt_log

[linking process]: https://defmt.ferrous-systems.com/setup#linker-script

<!-- This is interesting but seems too much for such a basic tutorial. -->
<!-- Let's first briefly discuss the topic of _logging_. -->
<!---->
<!-- Since `println!`, `print!` and `dbg!` are provided by `std`, which we don't have, we need to provide them. -->
<!---->
<!-- These macros are provided by `esp_println`. For [example, here](https://github.com/esp-rs/esp-hal/blob/9e4c652d1aa1d1cbc8f2483c93b7d98d0ba1bcb6/esp-println/src/lib.rs#L40): -->
<!-- ```rust -->
<!-- /// Prints to the selected output, with a newline. -->
<!-- #[cfg(not(feature = "no-op"))] -->
<!-- #[macro_export] -->
<!-- macro_rules! println { -->
<!--   // ... -->
<!-- ``` -->
<!---->
<!-- These macros are available without any of the loggers enabled. -->
<!---->
<!-- There are many loggers we could add, besides that provided by `esp_println`. For example, we could select `log`. This one, requires us to implement `log::Log` trait. Something like: -->
<!---->
<!-- ```rust -->
<!-- // Some struct -->
<!-- struct EspLogger; -->
<!-- impl log::Log for EspLogger { -->
<!--     // ... -->
<!-- ``` -->
<!---->
<!-- When adding the feature `log-04`, the crate `esp_println` will enable an implementation of that logger. And that will be used for `log::info!` and the other macros. All of them, in their implementation, use [their `println!` macro.](https://github.com/esp-rs/esp-hal/blob/9e4c652d1aa1d1cbc8f2483c93b7d98d0ba1bcb6/esp-println/src/logger.rs#L86) -->
<!---->
<!-- A similar situation happens with `defmt`: -->
<!---->
<!-- ```rust -->
<!-- #[defmt::global_logger] -->
<!-- pub struct Logger; -->
<!-- unsafe impl defmt::Logger for Logger { -->
<!--     // ... -->
<!-- ``` -->
<!---->
<!-- But in this case, there is no use of the macro (it uses some more efficient mechanism). -->
<!---->
<!-- [`defmt`][defmt] is an efficient logging framework. Just like `log`, the `defmt` crate provides `defmt::info`, `defmt::warn` and other macros. It also has a `defmt::println!` macro. -->
<!---->
<!-- The name `defmt` comes from *deferred formatting*: -->
<!-- > (...) instead of formatting `255u8` into `"255"` and sending the string, the single-byte binary data is sent to a second machine, the host, and the formatting happens there. -->
<!---->
<!-- So the formatting is *deferred* to the host. The other bit improving efficiency is *compression*: -->
<!---->
<!-- > `defmt`’s string compression consists of building a table of string literals, like `"Hello, world"` or `"The answer is {:?}"`, at compile time. At runtime the logging machine sends *indices* instead of complete strings. -->
<!---->
<!-- Source: [defmt docs]. -->
<!---->
