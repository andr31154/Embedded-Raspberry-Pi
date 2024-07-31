The **Raspberry Pi Pico W** was used for this project, which is a flexible, low-cost board.
The MCU powering it is a **RP2040**, which is an **Arm Cortex M0+**.

For compiling:

Run the this command in terminal:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```shell
sudo apt-get install libudev-dev
```
### `elf2uf2-rs`

This tool is needed to be able to program the board over USB. In order to install it, run
the following in your terminal:

```shell
cargo install elf2uf2-rs
```

Then, run `elf2uf2-rs --help`. If it was correctly installed, you should see something similar
to this in your terminal:

```shell
Usage: elf2uf2-rs [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input file
  [OUTPUT]  Output file

Options:
  -v, --verbose  Verbose
  -d, --deploy   Deploy to any connected pico
  -s, --serial   Connect to serial after deploy
  -h, --help     Print help
```

### Compiling

You will need to compile your executable specifically for the **RP2040** chip. The 
**ARM Cortex M0+** is based on the `Thumbv6m` architecture, so we will need to specify
our target when compiling. We can do that in multiple ways:

* using a `.cargo/config.toml` file:

```toml
[build]
target = "thumbv6m-none-eabi"
```

* passing it as a parameter to Cargo:

```shell
cargo build --release --target thumbv6m-none-eabi
```

On our repository, the first solution is already implemented, and you will only need to run 

```shell
cargo build --release --bin <task_binary_name>
```

### Flashing

To flash a program to the Raspberry Pi Pico via USB, it needs to be in *USB mass storage device mode*.
To put it in this mode, you need to **hold the `BOOTSEL` button down**  while connecting it to your PC.
Connecting and disconnecting the USB can lead to the port getting damaged, so we conveniently attached
a reset button on the breadboard included on the **Pico Explorer Base**. Now, to make it reflashable
again, just press the two buttons simultaneously.

After connecting the board to your PC and compiling the program, locate the binary in the
`target/thumbv6m-none-eabi/release/` folder then, run:

```shell
elf2uf2-rs -d -s /path/to/your/binary
```

* `-d` to automatically deploy to a mounted pico
* `-s` to open the pico as a serial device after deploy and print serial output

# TASKS

### Blink 💡

For this task, at least two of the three color channels of the `RGB` LED provided
to the microcontroller will need to be connected. A resistor has to be added for
each channel. The LEDs have a common anode.

One LED should toggle when the button located on the Pico Base Explorer is pressed,
and the other one will toggle every second.

Firstly, two output channels for the Red and Green LEDs have to be set up. Then,
an input channel for Button A for interacting with the board. The USB driver is
initialized and spawns the logger task. In a similar fashion two tasks for
`blink_on_timer` and `blink_on_press` are spawned. A timer that will wait
for 1 second is then set, then the LED is toggled.

In the end the programme waits for the button to be pressed, then toggles the LED.

### Discover 🔍

The server this was run on was running on `192.168.1.199:3000`. The task's purpose
is to send a specific code to it ("11OP63D3"). In exchange, the programme receives
a message that is then printed to serial. 

A logger and a WiFi task have to be spawned. The control peripheral on the CYW43 chip
is initialized. A `Config` is created using a static address.
Then we have to connect to the access point using the Wyliodrin network with the
password "g3E2PjWy".

After successful connection to the server, it sends to the server the code (11OP63D3),
and prints the response received to the USB. The message has to be then decoded :).
