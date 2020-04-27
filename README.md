# MSP430FR4133_Temperature
##### A low level project for the MSP430FR4133 written in Rust.

[![Build Status](https://travis-ci.com/stianeklund/MSP430FR4133_Temperature.svg?branch-master)](https://travis-ci.com/stianeklund/MSP430FR4133_TEMPERATURE)

Displays the temperature on the MSP430FR4133 LCD.
Previous versions of this project did not really do anything, that is to say I ended up
doing something else instead of finishing it..

It's now been brought up to date to use `Rust 2018 edition` and 
uses the newer [msp430](https://crates.io/crates/msp430) & [msp430-rt](https://crates.io/crates/msp430-rt) 
crates.

Uses the [msp430fr4133 peripheral access crate](https://crates.io/crates/msp430fr4133) 
which was generated by [svd2rust](https://docs.rs/svd2rust/0.17.0/svd2rust/) 
& [msp430_svd](https://github.com/pftbest/msp430_svd) for device level access.


System interrupts are handled by `msp430-rt` and at the moment no other interrupts are implemented.

This project does not rely on any external hardware abstraction libraries.
FYI: There is no HAL for msp430 (yet), however chances are the cortex-m HAL can be ported. 🙂

### How to build

Install the latest msp430-gcc toolchain Texas Instruments: `http://www.ti.com/tool/MSP430-GCC-OPENSOURCE`


Linux:

```
wget http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/8_3_2_2/export/msp430-gcc-8.3.1.25_linux64.tar.bz2 -O /tmp/msp430-gcc.tar.bz2
tar -xvjf /tmp/msp430-gcc.tar.bz2
export PATH=$PATH:$PWD/msp430-gcc-8.3.1.25_linux64/bin
```
Windows:

Download & export msp43-gcc for Windows:
```
http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/8_3_2_2/export/msp430-gcc-8.3.1.25_win64.zip
```
Note: Make sure to update your environment PATH variable to include the bin path.
Alternatively run the installer from: https://gnutoolchains.com/download/

#### Using cargo-xbuild

```
cargo install cargo-xbuild
rustup component add rust-src
```

Producing the elf file:

`cargo xbuild --target msp430-none-elf`

Once you've compiled you'll want to load the elf onto your msp430. I recommend using mspdebug: `https://github.com/dlbeer/mspdebug`
`mspdebug` is also available on Linux in many of the built-in repositories.

Note: `mspdebug` does _not_ work on `WSL` due to limitations with USB device access.

##### Loading the elf file onto your msp430:

* Use `mspdebug tilib` or `mspdebug ezfet`
* `mspdebug load targetfile` and read the on-screen instructions.

---

#### Notes on functionality:

* Displays the temperature from ADC12 on the LCD.
* Ability to write numbers to the LCD.
* Ability to write strings to the LCD (currently scrolling is not
supported).

Please keep in mind:
* This is not a complete project, it was used as a learning experience writing Rust code for the msp430.
* No interrupt handling aside from system hardware interrupts (which essentially are infinite loops).
* The LCD implementation could be _much_ better.