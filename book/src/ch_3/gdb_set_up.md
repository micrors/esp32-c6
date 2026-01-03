# Debugger - Set Up

Programs are debugged using:

- Printing value of a variable (e.g. `println!`).
- Asserting the value of a variable (e.g `assert!`).
- Writing unit tests.

To access finer execution details, we can use a _debugger_.

## GDB and OpenOCD

Two programs will be needed:

- GDB: with _multiarch_ to support riscv32 architecture.
    - Linux: `sudo apt-get install gdb-multiarch`.
    - MacOS: `brew install gdb` (it's multiarch).
- OpenOCD: handles the communication with the board.
    - The installation is described below.

Download the latest `openocd` for your laptop architecture from the `openocd-esp32` [github repo]. The repo is a fork of `openocd` targeted at `esp32` boards.

> [!IMPORTANT]
> Please, check the commands before executing them.

- For Linux AMD-64:
    ```bash
    # Linux amd64. See releases page for other archs.
    cd ${HOME} # go to home so we download in a visible place.
    OPENOCD_ZIP_NAME=openocd-esp32-linux-amd64-0.12.0-esp32-20250707.tar.gz
    DATE=v0.12.0-esp32-20250707
    wget https://github.com/espressif/openocd-esp32/releases/download/${DATE}/${OPENOCD_ZIP_NAME}
    tar -xvf ${OPENOCD_ZIP_NAME}
    ```
- For MacOS arm64:
    ```bash
    cd ${HOME}
    OPENOCD_ZIP_NAME=openocd-esp32-macos-arm64-0.12.0-esp32-20250707.tar.gz
    DATE=v0.12.0-esp32-20250707
    wget https://github.com/espressif/openocd-esp32/releases/download/${DATE}/${OPENOCD_ZIP_NAME}
    tar -xvf ${OPENOCD_ZIP_NAME}
    ```

> [!IMPORTANT]
> Below, it is assumed the output of the command above was a directory named `openocd-esp32`.

## Modify the PATH

Let's add a path to the `PATH` variable, by editing the `.zshrc` or `.bashrc` file.

Additionally, the `OPENOCD_SCRIPTS` variable is defined. `openocd` uses that variable to find the board's configuration.

```bash
# add `openocd` to PATH
export PATH=${HOME}/openocd-esp32/bin:${PATH}
# Define openocd scripts/configs location.
export OPENOCD_SCRIPTS="${HOME}/openocd-esp32/share/openocd/scripts"
```

Then source the profile:

```bash
source ~/.bashrc # or ~/.zshrc
```

Confirm installation with `openocd --version`. Now to some exercises.

[github repo]: https://github.com/espressif/openocd-esp32/releases
