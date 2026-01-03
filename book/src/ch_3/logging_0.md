# Logging - Part 1

What happens if we use the boilerplate, and no logging dependency?

In the `src/main.rs` we start with:

```rust
#![no_std]
#![no_main]
// None of these: esp_backtrace, esp_println, defmt
use esp_hal::{
    main,
    Config,
};
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_hal::init(Config::default());
    panic!("Welcome.")
}
```

Running `cargo run --release`, we get:
> error: `#[panic_handler]` function required, but not found

The panic handler is something we can define, or use a crate. For simplicity, let's use `esp-backtrace` with that feature, adding to `Cargo.toml` the following dependency:

```toml
esp-backtrace = { version = "0.18.1", features = [
    "esp32c6",
    "panic-handler",
 ]}
```

And to `src/main.rs`:

```rust
// ...
use esp_backtrace as _;
// ...
```

Re-run `cargo run --release`, and we get a new error:
> error: failed to run custom build command for `esp-backtrace v0.18.1`
> ...
> Exactly one of the following features must be enabled: defmt, println

So we add one:

```diff
esp-backtrace = { version = "0.18.1", features = [
    "esp32c6",
    "panic-handler",
+   "println"
]}
```

And yet, it errors:

> error: failed to run custom build command for `esp-println v0.16.1`
>
> Exactly one of the following features must be enabled: jtag-serial, uart, auto, no-op

This, it seems, would require that we add `esp-println`, and enable some of those features.

Since `auto` is a default feature, we can add:

```toml
esp-println = { version = "0.16.1", features = [
    "esp32c6",
] }
```

Now, it will run and panic. Yay!

Playing with these dependencies helps get a sense of what they are for. Also, to see how useful the compiler error messages can be.

## Print something

So far we've not printed anything, but given `esp-println` is already there, we only need to `use` it. Then we can `println!("Something")` and it should work.

At this step, the code looks like:

```rust
#![no_std]
#![no_main]
use esp_hal::{
    main,
    Config,
};
use esp_backtrace as _;
use esp_println::println;
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_hal::init(Config::default());
    println!("println!");
    panic!("Welcome.")
}
```

## Using the log crate

We add a few items:

```diff
use esp_backtrace as _;
+use esp_println::{logger,println};
+use log::info;
esp_bootloader_esp_idf::esp_app_desc!();
```

And within `main`:

```diff
fn main() -> ! {
    esp_hal::init(Config::default());
    +logger::init_logger_from_env();
    +info!("SOME INFO!");
    println!("println!");
```

Finally, on `Cargo.toml`:

```diff
esp-println = { version = "0.16.1", features = [
"esp32c6",
+"log-04"
] }
# Manuallu, or use `cargo add log`
+log = "0.4.29"
```

So now we have learnt how to add a logger, if we need to.

>[!NOTE]
> `examples/logging_0.rs` contains a solution. You can run it with the following command: `cargo run --example logging_0 --release`. You will need to have the settings above done correctly though!

In the next logging section, we analyse using `defmt` instead.
