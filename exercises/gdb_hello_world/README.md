# GDB exercise

## OpenOCD

Useful commands for `openocd`:

- `-f <board_cfg_file>`.
- `-d<n>`: 0-3 for off-verbose, respectively.
- `-l out.log`: puts the logs in that file.
- `-s`: directory to search for config files.

## GDB

A few shortcuts (`C` is Control key, `M` is Meta key.):

- Movements
    - `C-b` back, `C-f` forward (both 1 character).
    - `M-f` forward, `M-b` backward (1 word).
    - `C-a` start, `C-e` end (of the line).
- `C-r` search backwards, `C-s` search forwards in history.
- `DEL` or `Backspace` Delete the character to the left of the cursor.
- `C-d` Delete the character underneath the cursor.
- `C-x C-u` Undo the last editing command.
- `C-l` Clear the screen, reprinting the current line at the top
