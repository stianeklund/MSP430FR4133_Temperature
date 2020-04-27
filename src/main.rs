#![feature(llvm_asm)]
#![no_std]
#![no_mangle]
#![no_main]

mod adc;
mod lcd;
mod lcd_helper;

use crate::adc::*;
use crate::lcd::*;
use msp430_rt::entry;
use msp430fr4133::Peripherals;
use panic_msp430;

#[warn(overflowing_literals)]
#[warn(arithmetic_overflow)]
#[entry]
fn main() -> ! {
    let mut dp = Peripherals::take().unwrap();
    let wdt = dp.WATCHDOG_TIMER;

    // Disable watchdog
    wdt.wdtctl
        .modify(|_, w| w.wdtpw().password().wdthold().set_bit());

    // Setup XT1 Oscillator
    dp.PORT_3_4
        .p4sel0
        .modify(|_, w| w.p4sel0_1().set_bit().p4sel0_2().set_bit());

    dp.TIMER_0_A3.ta0cctl0.modify(|_, w| w.ccie().set_bit());
    dp.TIMER_0_A3.ta0ccr0.write(|w| unsafe { w.bits(0xc350) });
    dp.TIMER_0_A3
        .ta0ctl
        .modify(|_, w| w.tassel().tassel_2().mc().mc_2());

    // Oscillator flag fault , clear bits
    while dp.SFR.sfrifg1.read().ofifg().bit_is_set() {
        dp.CS
            .csctl7
            .modify(|_, w| w.xt1offg().clear_bit().dcoffg().clear_bit());
        dp.SFR.sfrifg1.modify(|_, w| w.ofifg().clear_bit());
    }
    // XT1 Oscillator (set highest drive mode)
    // Needs to be reconfigured if waking up from LMP3.5
    dp.CS.csctl6.modify(|_, w| w.xt1drive().xt1drive_3());

    // Disable the GPIO power-on default high-impedance mode
    // needs to be disabled to activate previously configured port settings.
    dp.PMM.pm5ctl0.write(|w| w.locklpm5().clear_bit());

    // Set LCD power pin on in System Config register
    dp.SYS.syscfg2.modify(|_, w| w.lcdpctl().set_bit());
    lcd_init(&mut dp.LCD_E);
    init_adc(&mut dp.ADC);

    // Write PMM password to unlock PMM registers
    dp.PMM
        .pmmctl0
        .modify(|_, w| w.pmmpw().password().pmmregoff().set_bit());
    dp.SYS.syscfg0.modify(|_, w| w.pfwp().set_bit());

    // Temperature sensor enable, set internal reference &  set reference bandgap
    dp.PMM.pmmctl2.modify(|_, w| {
        w.tsensoren()
            .set_bit()
            .intrefen()
            .set_bit()
            .refbgrdy()
            .set_bit()
            .refgenrdy()
            .set_bit()
            .refbgact()
            .set_bit()
    });

    clear_lcd(&mut dp.LCD_E);

    loop {
        dp.PORT_1_2.p1out.modify(|_, w| w.p1out0().clear_bit());
        poll_temp(&mut dp.ADC, &mut dp.PORT_1_2, &mut dp.LCD_E);

        delay(1000);
    }
}

fn delay(n: u32) {
    unsafe {
        let mut i = n;
        while i > 0 {
            i -= 1;
            llvm_asm!("nop": : : : "volatile");
        }
    }
}
