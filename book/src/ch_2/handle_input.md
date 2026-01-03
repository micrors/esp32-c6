# Handle Input

Using same wiring from the `blinky` chapter, let's now use external input to make the LED blink.

- Access `exercises/handle_input` in order to edit `src/main.rs`.

> [!WARNING]
> This exercise is slightly more involved, in the sense you'll need to add more code. But there is always the solution if it gets too hard.

## Exercise

We will use the button labelled [`BOOT` linked to `GPIO9`][Button-GPIO9] to toggle the LED.

0. As per usual, we will be editing `src/main.rs`.
1. We first need to fill in the boilerplate. Once that's done, this line might be of help:
    ```diff
    // you will also have other `esp_hal` submodules besides `gpio`.
    +use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    ```
    You may get inspired by the previous exercises and the boilerplate page.

2. Create an `Output::new` passing GPIO7 peripheral, `Level::High` and default `OutputConfig`.
   - Assign it to the `led` variable.
   - Hint: with _peripherals_ initialised, GPIO7 is a field in that struct.
3. Create an `Input::new` passing GPIO9 peripheral.
   - Use default `InputConfig`, overwrite with `as_pull(Pull::High)`.
   - Assign it to the `btn` variable.
4. Add the logic inside `loop`:
   - When pressing it should turn the `led` on, and delay it 2 seconds.
   - Then it turns itself off.

The `examples/handle_input.rs` contains a solution. You can run it with the following command `cargo run --example handle_input --release`.

[Button-GPIO9]: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html#id15
