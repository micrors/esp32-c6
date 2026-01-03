# Panic

If something goes very wrong, a program will `panic!`.

[esp-backtrace] provides a `panic!` implementation through the `panic-handler` feature.

"Handling panic!" here means to print what was executed up to that point (the backtrace), and probably stops code execution as well.

We could use our own panic-handler instead, and no backtrace at all. For example, [this one][panic_handler]:

```rust
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
```

> [!warning]
> We need to `use esp-backtrace as _;` so that this handler is included in the final binary.

In the exercise below, we configure a _profile_. Profiles are configurations `cargo` uses to control compilation. When unspecified, `cargo` sets sensible defaults for us.

## Exercise 1

0. Access the project at `exercises/panic`.
1. Add all the boilerplate code described earlier.
2. Add a `panic!` within `main`.
   - But remember, it's for irrecoverable errors!
3. Run the code with `cargo run`; this uses the _development_ profile.

   - It outputs debug information into the compiled binary.
4. Then run with _release_ profile `cargo run --release`.
   - The default `--release` behaviour _excludes_ debug information and minimises the binary size; the backtrace shows the missing debug information with `??`.

       ```console
        Hello world!
        ====================== PANIC ======================
        panicked at examples/panic.rs:24:5:
        This is a panic
        Backtrace:
        0x4200252a
        main
            at ??:??
        ```

`examples/panic.rs` contains a solution. It can be run with: `cargo run --example panic --release`.

## Exercise 2

1. Edit the `.cargo/config.toml`:
    ```diff
    +[profile.release]
    +debug = true
    ```

   - Now it will emit debug information in the ELF binary file; _yet debug info isn't flashed_ into the target, it is just used to display the backtrace.
   - Re-run the program with `--release` and confirm `??:??` is now filled in.
2. Try another option:
    ```diff
    +[profile.dev]
    +opt-level = "s"
    ```

    which turns size-optimisation on, and has debug info by default. Run it with `cargo run` (no `--release` flag since we want to run the dev profile.)

## Recap

- Using `esp-backtrace`.
- Using `panic!` to exit the program on error.
- Profiles:
    - Tweak debug information on backtrace.
    - Optimise binary size.

## Suggested Reading

- [esp-backtrace].

[esp-backtrace]: https://docs.rs/crate/esp-backtrace/latest
[panic_handler]: https://esp32.implrust.com/std-to-no-std/panic-handler.html
