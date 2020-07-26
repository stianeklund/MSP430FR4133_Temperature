use crate::delay;
use crate::lcd_helper::{write_char_pos, write_temp};
use msp430fr4133::{ADC, LCD_E, PORT_1_2};

pub fn init_adc(adc: &mut ADC) {
    adc.adcctl0.modify(|_, w| {
        w.adcsht()
            .adcsht_8() // ADC Sample select 8
            .adcon()
            .set_bit() // Turn ADC ON
    });

    adc.adcctl1.modify(|_, w| w.adcshp().set_bit());
    // 10-bit conversion result
    adc.adcctl2.modify(|_, w| w.adcres().adcres_1());

    adc.adcmctl0.modify(|_, w| {
        w.adcinch()
            .adcinch_12() // ADC Input Channel 12 (Temperature sensor)
            .adcsref()
            .adcsref_1() // Select Reference 1
    });
    adc.adcie.modify(|_, w| w.adcie0().set_bit()); // Interrupt request for ADC_B conversion
}

#[warn(overflowing_literals)]
pub(crate) fn poll_temp(adc: &mut ADC, port: &mut PORT_1_2, lcd: &mut LCD_E) {
    let caladc30c: u16 = 0x1a1a; // Ref calibration @30C 0x1A1A
    let caladc85c: u16 = 0x1a1c; // Ref calibration at @85C 0x1A1C

    delay(9000);

    // Turn ADC ENC on & no conversion
    adc.adcctl0
        .modify(|_, w| w.adcenc().set_bit().adcsc().clear_bit());

    adc.adcmctl0.modify(|_, w| {
        w.adcinch()
            .adcinch_12() // ADC Input Channel 12 (Temperature sensor)
            .adcsref()
            .adcsref_1() // Select Reference 1
    });

    /*
    let temp = (i32::from(adc.adcmem0.read().bits())
    .wrapping_mul(27069_i32.wrapping_sub(18169625_i32))
    .wrapping_shr(16));
    */

    // Info about the conversion was found: http://www.ti.com/lit/ug/slau445i/slau445i.pdf

    adc.adcmem0.read().bits();
    let temp = adc.adcmem0.read().bits().wrapping_sub(caladc30c);

    let temp_c = (temp
        .wrapping_mul(10)
        .wrapping_mul(85_u16.wrapping_sub(30).wrapping_mul(10)))
    .wrapping_div(caladc85c.wrapping_sub(caladc30c).wrapping_mul(10))
    .wrapping_add(300);

    adc.adcctl1
        .modify(|_, w| w.adcshs().adcshs_2().adcconseq().adcconseq_2());

    if adc.adcctl1.read().adcbusy().bit_is_set() {
        // Turn ADC ENC off
        adc.adcctl0.modify(|_, w| w.adcenc().clear_bit());
        delay(100);
    }

    // Start conversion
    adc.adcctl0
        .modify(|_, w| w.adcenc().set_bit().adcsc().set_bit());
    adc.adcctl1.modify(|_, w| w.adcshp().set_bit());

    // TODO: There is something funky about the values we are getting from the ADC
    // or with ADC conversion.
    // Perhaps the lack of averaging the values here is causing the big fluctuation in readings?

    write_temp(temp_c as u16, lcd);
    write_char_pos('C', 6, lcd);
    // Display ° Degree symbol in segment 6
    lcd.lcdm2w.modify(|_, w| unsafe { w.bits(0x0400) });

    // TODO: Interrupt handling
    // Enable interrupt request for a completed ADC conversion
    adc.adcie.modify(|_, w| w.adcie0().set_bit());
    port.p1dir.modify(|_, w| w.p1dir0().set_bit());
    port.p1out.modify(|_, w| w.p1out0().set_bit());
    delay(12000);
}
