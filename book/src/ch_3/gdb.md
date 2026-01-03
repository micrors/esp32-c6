# Debugging - Exercises

Now that `gdb` or `gdb-multiarch` are set up, let's do some exercises.

1. Access `exercises/gdb_hello_world`
2. Inspect the configuration files `openocd.cfg` (for `openocd` ) and `.gdbinit` (for `gdb`).
3. Execute `cargo run`, leave out `--release` to use the development profile.
   - Without it, cargo will remove / optimise lines.
   - We _should_ debug with `--release` to ensure the best outcomes.
   - But this examples is just to get started.
4. Run `gdb -x gdbinit -q` (or `gdb-multiarch` on Linux) in one terminal, and `openocd` in another (at the same directory!).
   - In the image, `_peripherals` has not yet been assigned. `gdb` stopped execution there.
      The window at the bottom half is our `(gdb)` prompt.
      <div class="center w420">
      <a href="../assets/gdb.png">
      <img src="../assets/gdb.png" alt="GDB screenshot showing our main.rs source code. It has a B where the breakpoint is."/>
      </a>
      </div>
5. Add a `break` point where `a/=2`, then `print` the resulting value.
6. Exit with `Ctrl+D` or `Ctrl+C`.

## Suggested Reading

- [MB2] article on Debugging.

[MB2]: https://docs.rust-embedded.org/discovery-mb2/05-meet-your-software/debug-it.html
