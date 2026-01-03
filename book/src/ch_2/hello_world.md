# Hello World

There is a lot to learn, so let's start simple. Our "hello world" is more of a logging practice.

1. Connect the board to your computer and access the first project:

    ```bash
    cd exercises/hello_world
    ```

2. *Build, flash, and monitor* the project with `cargo run`.
    You should see:

    ```txt
    ..
    Commands:
        CTRL+R    Reset chip
        CTRL+C    Exit
    ..
    loop..!
    ..
    ```

    >[!TIP]
    > Try pressing `CTRL+R` to reset, then press `CTRL+C` to exit.

    The aim here is just run the binary.

## *Build, flash and monitor*?

The `.cargo/config.toml` includes this config:

```toml
[target.riscv32imac-unknown-none-elf] # our processor arch.
runner = "espflash flash --monitor"   # for `cargo run`.
```

Our `cargo run` is replaced by `cargo build && espflash flash <elf_path> --monitor`.

This builds and then flashes the binary. Finally, it monitors for logs.
Removing `--monitor` it won't wait for logs.

## Exercise

We already know our code can print information with `println!` macro. For prettier logs we can use `log` or `defmt`. Let's try adding them:

0. Take some time to read the code. Do you recognise the boilerplate discussed earlier?
1. Try commenting out the line:
    ```rust
    use esp_backtrace as _;
    ```
    So that we can see what it provides. Then uncomment the line.
2. Uncomment the "log lines" in `src/main.rs`,
3. Add the `log` dependency to `Cargo.toml`.
4. The `log` crate logging level is controlled with `ESP_LOG` under the `[env]` section in `.cargo/config.toml`.
   - Change the `ESP_LOG` variable to turn `off` all logs. Re-run `cargo run`, to test how it works.

   - Try with other levels, for example, with `trace`.

The `examples/hello_world.rs` contains a solution.
You can run it with `cargo run --example hello_world`.

> [!IMPORTANT]
> Running the solution requires fixing the log-lines at the bottom of `Cargo.toml`.

## Suggested Reading

- [esp-println] crate's docs.
- [log] crate's docs.

[esp-println]: https://docs.rs/esp-println/latest/esp_println/
[log]: https://docs.rs/log/latest/log/
