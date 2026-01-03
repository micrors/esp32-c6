# This book

This is an embedded-rust tutorial using the esp32-c6 board.

The book aims to be self-contained, but brief reading material will be suggested.

The goals are to show:

1. The basics of the board and hardware components,
2. How to start and configure new projects,
3. What we can do with a board.

so that you can do projects independently afterwards.

## Prior Knowledge

Rust knowledge is assumed, but no embedded experience is required.
The content was written for Unix (Linux, MacOS).

## Hardware

The esp32-c6 board is available on Mouser, Aliexpress and other retailers.
 It should look similar to:

<div class="center w220">
    <a href="./assets/board.jpg">
    <img  alt="Close up image of our board." src="./assets/board.jpg"/>
    </a>
</div>

Other required hardware:

- USB-C cable to link your computer and the board.

    - The cable *has to* support data transfers and not be power-only.

- One resistor, 2 jumper wires, one LED, one breadboard.
These are needed for *Blinky* and *Handle Input* chapters.

## Source

The source for the **book** and **code exercises** is at <https://github.com/micrors/esp32-c6>.
To get started, clone repo and then change directory:

    git clone https://github.com/micrors/esp32-c6 micrors_esp
    cd micrors_esp

**Repository contents**:

- `book/`: markdown sources of this book.
- `exercises/`: code examples and exercises.

<!-- Too much info. May be OK for an appendix.
* [Code examples] for setting up peripherals
* [MCU datasheet] i.e for the esp32-c6 micro controller.
* [reference manual] for esp32-c6 board: large, complete description of
board.
[MCU datasheet]: https://documentation.espressif.com/esp32-c6-wroom-1*wroom-1u*datasheet_en.pdf

[reference manual]: https://documentation.espressif.com/esp32-c6*technical*reference*manual*en.pdf

[Code examples]: https://github.com/esp-rs/esp-hal/tree/main/examples
-->
