# GDB Commands

GDB basic commands, classified with respect to their purpose.

## Program Flow

- `next` step one line over.
- `continue` or `c`: continue executing up to next break point.
- `break fn_name`: break point at function name.
    - There may be many with same name. So we need the path.
        - For example: `break gdb_hello_world::__risc_v_rt__main`
- `break lineno`: break point at line number of currently-focused file.
    - Example: `break 19`, or `br 19`.
- `break my_file.rs:lineno`: example `break main.rs:17`.
- `monitor reset halt`: restarts and halts it.
- `load`: if we only use `cargo build`, then we can `flash` using `load`.
    - Within `gdb` type `load`. It'll flash the file to the MCU.

## Layout

- `layout src`: shows source code and CLI.
- `layout asm`: shows assembly source and CLI.
- `tui disable`: to disable the layout.

## Overviews

- `info break`: to show breakpoints
- `info locals`: to show variables.
- `print x`: to print variable `x`. Also `print &x` prints the address of `x`.
- `list` or `list main`: will show our program with numbered lines.
