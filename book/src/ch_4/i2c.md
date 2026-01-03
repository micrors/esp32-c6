# I2C

I2C (inter-integrated communication) is a _protocol_ and also a _peripheral_. We are concerned with the _protocol_.

"inter" in the name refers to _within_ the board, "integrated" refers to integrated-circuits. In other words: communication between multiple integrated circuits within a board (or short distances).

Yet, devices supporting I2C need hardware that understands the protocol rules. Here, we will focus on its usage and most relevant concepts.

> [!NOTE]
> An integrated circuit (IC) is an extremely packed, microscale version of a circuit. We could build one with wires, resistors and other components (and tons of effort).
>
> ICs are the "black squares" in the PCB, but are not always visible, for example those within the MCU, or inside a sensor.

I2C is within the Synchronous Serial Communication umbrella:

- It is Serial because it transmits one bit at a time.
- It uses two lines: a serial clock-line (SCL) and a serial data-line (SDA).
- The SCL makes it a synchronous protocol.

The complete diagram is in [this document][TI I2C] (section 2.1), but a similar one from Wikipedia is shown below:

<div class="center w320"> <!--other classes: w220, w420-->
<!--Start with `/` following path from `src`.-->
    <a href="../assets/i2c_wikipedia.svg">
        <img alt="Diagram showing the two wires SDA, SCL and many devices linked to them." src="../assets/i2c_wikipedia.svg"/>
    </a>
    <p> By <a href="https://commons.wikimedia.org/w/index.php?title=User:Tim_Mathias&amp;action=edit&amp;redlink=1" class="new" title="User:Tim Mathias (page does not exist)">Tim Mathias</a> - <span class="int-own-work" lang="en">Own work</span>, <a href="https://creativecommons.org/licenses/by-sa/4.0" title="Creative Commons Attribution-Share Alike 4.0">CC BY-SA 4.0</a>, <a href="https://commons.wikimedia.org/w/index.php?curid=111975651">Link</a></p>
</div>

- **SDA** is the Serial Data line, the road data travels on (in any direction). **SCL** is the Serial Clock line, which times when to read-from or write-to SDA.

- The **Vdd** line pulls up the voltage in the SDA and SCL lines. There is a **GND** line as well, omitted here.

- All the coloured blocks are devices, 3 can be targets only, one can be controller only. `µC` means microcontroller, and Rp are resistors.

The last bit of terminology are the _roles_:

- Controller (formerly _master_): initiates communication. Microcontrollers are always in the controller role.
- Target (formerly _slave_): the target of the controller. Most devices &ndash;except MCUs&ndash; are targets.

And the _modes_: whichever the role is, a device can either be the receiver or the transmitter of data.

## Communication

The high level view of the protocol is:

1. A controller broadcasts START bit.
2. It then broadcasts a _target device address_ (usually 7 bits).
3. Then sends a single bit where 0 receiver, 1 means transmitter.
4. If the broadcasted _address_ matches the ID of a target, the target replies with an acknowledgement and the communication starts.
5. Now 8 bits of _data_ are sent (either direction). The receiver always sends an acknowledgment bit.
   - This step can repeat many times.
6. No other device will transmit data during this time. It's "locked".
7. Finally the controller broadcasts a STOP bit, and the bus is unlocked.

Here, most bits are sent through SDA, but the SCL also helps define when communication starts and ends, and times when to read from SDA.

The speed of the communication is a bit faster than UART's deppending on the mode, in the order of 10 KBps to 100 KBps.

## For curiosity

- Search for "OLED I2C", or just any sensors. Many will support this or some of the already-learnt protocols. For I2C there will be a Vdd, GND, SDA, SCL lines, as expected.
- Don't buy more stuff! It's just to learn. You may even have some sensor supporting I2C already.

## Suggested Reading

<!-- - [Basic concept of IC][IC] -->

- [I2C Protocol]: explaions the protocol in a bit more detail than done here.
- [Texas Instruments I2C][TI I2C] subsection 2.1 (specially the schematic they have is quite good).

<!-- [IC]: https://learn.sparkfun.com/tutorials/integrated-circuits/all -->
[I2C Protocol]: https://www.circuitbasics.com/basics-of-the-i2c-communication-protocol/
[TI I2C]: https://www.ti.com/lit/an/sbaa565/sbaa565.pdf
