#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(asm)]

extern crate volatile_register;
use volatile_register::RW;
use core::fmt;
use core::fmt::{Display, Formatter};
use core::str;

extern "C" {
    // ADC Channel 0
    static mut ADCCTL0:    RW<u16>;
    static mut ADCCTL0_L:  RW<u16>;
    static mut ADCCTL0_H:  RW<u16>;

    // ADC Channel 2 Control
    static mut ADCCTL1:    RW<u16>;
    static mut ADCCTL1_L:  RW<u16>;
    static mut ADCCTL1_H:  RW<u16>;

    // ADCMCTL0 Control Bits
    static mut ADCMCTL0:   RW<u16>;
    static mut ADCMCTL0_L: RW<u16>;
    static mut ADCMCTL0_H: RW<u16>;

    // ADC Conversion Memory
    static mut ADCMEM0:    RW<u16>;
    static mut ADCMEM0_L:  RW<u16>;
    static mut ADCMEM0_H:  RW<u16>;

    // Watchdog
    static mut  WDTCTL:    RW<u16>;

    // Power Management
    static mut PM5CTL0:    RW<u16>;
    static mut PMMCTL2:    RW<u16>; // 0x0008 for Temp sensor on
    static mut PBOUT_H:    RW<u8>;
    static mut PBDIR_H:    RW<u8>;
}

#[no_mangle]
#[link_section = "__interrupt_vector_reset"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = main;

fn temp_init() {
    unsafe {
        ADCCTL0.write(0x0002);          // ADC Enable conversion
        ADCCTL0.write(0x0010);          // Select Reference 1
        ADCCTL0.write(0x0300);          // ADC Sample Hold Select 3
        ADCMCTL0.write(0x0010);         // ADC Input Channel 10
        ADCCTL0.write(0x8000);          // ADC Sample Hold Select Bit: 3
        ADCCTL1.write(0x0060);          // ADC Clock Divider Select 3
        PMMCTL2.write(0x0008);          // Turn temp sensor on

    }
}

#[allow(exceeding_bitshifts)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
pub unsafe extern "C" fn main() -> ! {
    WDTCTL.write(0x5A00 + 0x0080);  // Turn watchdog timer off

    PBDIR_H.write(0b0100_0001);     // Set direction for P4
    PM5CTL0.write(0x0130);          // Lock GPIO
    PBOUT_H.write(0x01);

    ADCCTL0.write(0x0002);          // ADC Enable conversion
    ADCCTL0.write(0x0010);          // Select Reference 1
    ADCCTL0.write(0x0300);          // ADC Sample Hold Select 3
    ADCMCTL0.write(0x0010);         // ADC Input Channel 10

    ADCCTL0.write(0x8000);          // ADC Sample Hold Select Bit: 3
    ADCCTL1.write(0x0060);          // ADC Clock Divider Select 3
    PMMCTL2.write(0x0008);          // Turn temp sensor on

    ADCCTL0.write(0x0002 & 0x0001); // Enable & Start conversion

    let adc = ADCMEM0.read();       // Read ADC
    // Convert to temperature
    let temp = 27069 * adc - 18169625 >> 16;

    loop {
        PBOUT_H.modify(|x| !x);
        delay(100000);
        ADCMEM0.modify(|x| x);
    }
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn delay(mut n: u16) {
    unsafe {
        asm! {
            "1: \n dec $0 \n jne 1b" : "+r" (n) ::: "volatile"
        }
    }
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt() -> ! {
    loop {
        // Do nothing
    }

}
