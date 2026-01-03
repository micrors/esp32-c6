# Embedded Basics

The *board* (or DevKit), as shown earlier, is:
<div class="center w220">
    <a href="../assets/board.jpg"> <img  alt="Close up image of our board." src="../assets/board.jpg"/>
    </a>
</div>

The *board* then, is the whole item. Its main parts are:

- The *printed circuit board* (PCB): the flat fiberglass sheet supporting all electronic components.
- The *components*: items laid on top of the PCB. They interconnect through *traces* which are thin copper wires. Examples of components: *module*, USB ports, LEDs, GPIO-pin headers.

So we have the flat piece with the components laid on top. Before zooming further, this is the hierarchy we will be exploring in the definitions (the outer levels contain the inner levels)

- Board
    - Module
        - SoC (or MCU)
            - CPU, ROM, SRAM,
            - Advanced Peripherals: Wi-Fi, RF (Bluetooth).
            - Peripherals: GPIO, UART, I2C, ADC
        - Other Items
    - Components: USB, LED, GPIO pin-headers

The description below aren't technical, they are just an overview.

## Module

The *module* is the big square labelled ESP32-C6-WROOM (or some variant). It takes almost half the board.

Within it, a module has a few more items, the key one being the SoC, short for System on a Chip. We can't see the SoC unless we take the *module*'s lid off, but here is how it looks within:

<div class="center w320"> <!--other classes: w220, w420-->
<!--Start with `/` following path from `src`.-->
    <a href="../assets/devkit-module-soc.png">
        <img alt="Breakdown of Devkit (or just Board) to Module, to SoC." src="../assets/devkit-module-soc.png"/>
    </a>
    <p>Image from <a href="https://esp32.implrust.com/">implFerris/esp32-book</a> under <a href="https://creativecommons.org/licenses/by/4.0/">CC-BY-SA 4.0</a> </p>
</div>

## SoC

The SoC includes CPU, ROM, SRAM, advanced peripherals and peripherals. It's an all-in-one integrated circuit or chip.

The *advanced peripherals* are those making SoC an advanced MCU, like Bluetooth and Wi-Fi.

The *peripherals* are parts of the SoC placed on the *periphery* of the CPU, extending its capabilities. For example: GPIO, UART, I2C, SPI, DAC, ADC. We can't see them directly.

The GPIO peripheral is "broken out" into the pin headers at each side of the board. See the [user-guide] schematics and tables for the board.

### SoC or MCU?

In this tutorial we consider System on a Chip (SoC) and Micro controller unit (MCU) synonyms. Wikipedia defines MCU and disambiguates [SoC or MCU][soc_or_mcu] very well (I modified it slightly):

> A microcontroller (MC, uC, or μC) or microcontroller unit (MCU) is a small computer on a single integrated circuit. A microcontroller contains one or more processor cores along with memory and programmable input/output peripherals.
>
> In modern terminology, a microcontroller is similar to, but less sophisticated than, a system on a chip (SoC). A SoC may include a microcontroller as one of its components but usually integrates it with *advanced peripherals* like a graphics processing unit (GPU), a Wi-Fi module, or one or more coprocessors.

In very simple terms, a *module* contains the SoC which is just an advanced MCU.

## Putting it all together

An explanation of how it all comes together is in the [micro:bit v2][MB2] book, a modified version is provided below:

> Our MCU (or SoC) has tiny metal pins sitting right underneath it. These pins and the board components are wired via copper traces, the little "roads" on the board.
>
> The MCU can be programmed to change the electrical state of the pins. The pins change the electrical state of the traces, ultimately controlling the components. So *the MCU can control the components.*
>
> For example, by enabling or disabling electrical current to flow through a specific pin, an LED attached to that pin (via the traces) can be turned on and off.

## Suggested reading

- [esp32-c6 user-guide]. It describes the board components, peripherals, and pinouts.
- The [SoC datasheet], especially the product page, and the features (first 3 or 4 pages).
- [implrust]'s page on the distinction between DevKit (or Board), Module and SoC.

[esp32-c6 user-guide]: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitc-1/user_guide.html
[MB2]: https://docs.rust-embedded.org/discovery-mb2/04-meet-your-hardware/microbit-v2.html
[soc_or_mcu]: https://en.wikipedia.org/wiki/Microcontroller
[SoC datasheet]: https://documentation.espressif.com/esp32-c6_datasheet_en.pdf
[implrust]: https://esp32.implrust.com/esp32-intro/esp32-family.html
