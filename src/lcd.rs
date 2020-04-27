use crate::delay;
use crate::lcd_helper::write_string;
use msp430fr4133::LCD_E;

pub fn lcd_init(l: &mut LCD_E) {
    clear_lcd(l);
    l.lcdctl1.modify(|_, w| w.lcdfrmifg().set_bit());

    segment_enable(l); // Use 4 mux mode:(LCDM0 - LCDM39)

    // Set LCD_frequency divider / 3
    l.lcdctl0
        .modify(|_, w| w.lcdssel().lcdssel_0().lcddiv().lcddiv_2());

    // Set Mode 3, charge pump enable & set internal VREF: 3.08V
    l.lcdvctl.modify(|_, w| {
        w.lcdcpen()
            .set_bit()
            .lcdrefen()
            .set_bit()
            .lcdcpen()
            .set_bit()
            .lcdcpfsel0()
            .set_bit()
            .lcdcpfsel1()
            .set_bit()
            .lcdcpfsel2()
            .set_bit()
            .lcdcpfsel3()
            .set_bit()
            .vlcd()
            .vlcd_8()
    });

    // Turn LCD On
    l.lcdctl0
        .modify(|_, w| w.lcdson().set_bit().lcdon().set_bit());

    // Select pins L0 .. L4 as common line's
    l.lcdcssel0.modify(|_, w| {
        w.lcdcss0()
            .set_bit() // L0
            .lcdcss1()
            .set_bit() // L1
            .lcdcss2()
            .set_bit() // L2
            .lcdcss3()
            .set_bit() // L3
            .lcdcss4()
            .set_bit() // L4
    });

    // Clear LCD COM / SEG registers
    l.lcdcssel1.modify(|_, w| unsafe { w.bits(0x0000) });
    l.lcdcssel2.modify(|_, w| unsafe { w.bits(0x0000) });

    // L0 = COM0, L1 = COM1, L2 = COM2, L3 = COM3, L4 = COM4
    l.lcdm0w.modify(|_, w| unsafe { w.bits(0x8421) });

    // Set MUX rate bits
    l.lcdctl0
        .modify(|_, w| w.lcdmx1().set_bit().lcdmx0().set_bit());

    delay(90000);
    write_string("INIT", l);
    delay(90000);
}

// Zero out LCD memory
pub fn clear_lcd(l: &mut LCD_E) {
    l.lcdmemctl.modify(|_, w| w.lcddisp().clear_bit());
    l.lcdmemctl.modify(|_, w| w.lcddisp().clear_bit());
    l.lcdmemctl
        .modify(|_, w| w.lcdclrm().set_bit().lcdclrbm().set_bit());
    l.lcdm0w.modify(|_, w| unsafe { w.bits(0x8421) });
}

// Enable LCD Segments
fn segment_enable(l: &mut LCD_E) {
    l.lcdpctl0.modify(|_, w| unsafe { w.bits(0xffff) }); // LCDS Segments: 0-15
    l.lcdpctl1.modify(|_, w| unsafe { w.bits(0x7fff) }); // 16-30
    l.lcdpctl2.modify(|_, w| unsafe { w.bits(0x00f0) }); // 36, 37, 38, 39
    l.lcdpctl0
        .modify(|_, w| w.lcds0().set_bit().lcds1().set_bit());
}
