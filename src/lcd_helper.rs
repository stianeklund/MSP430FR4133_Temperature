use msp430fr4133::LCD_E;

pub fn lookup_char(c: char) -> u16 {
    match c {
        'a' | 'A' => 0x00EF,
        'b' | 'B' => 0x50F1,
        'c' | 'C' => 0x009C,
        'd' | 'D' => 0x50F0,
        'e' | 'E' => 0x009F,
        'f' | 'F' => 0x008F,
        'g' | 'G' => 0x00BD,
        'h' | 'H' => 0x006F,
        'i' | 'I' => 0x5090,
        'j' | 'J' => 0x0078,
        'k' | 'K' => 0x22E0,
        'l' | 'L' => 0x001C,
        'm' | 'M' => 0xA06C,
        'n' | 'N' => 0x826C,
        'o' | 'O' => 0x00F3,
        'p' | 'P' => 0x00CF,
        'q' | 'Q' => 0x02FC,
        'r' | 'R' => 0x02CF,
        's' | 'S' => 0x00B7,
        't' | 'T' => 0x5080,
        'u' | 'U' => 0x007C,
        'v' | 'V' => 0x280C,
        'w' | 'W' => 0x0A6C,
        'x' | 'X' => 0xAA00,
        'y' | 'Y' => 0xB000,
        'z' | 'Z' => 0x2890,
        _ => 0x0000,
    }
}

fn lookup_number(n: u16) -> u16 {
    match n {
        0 => 0x28FC,
        1 => 0x2060,
        2 => 0x00DB,
        3 => 0x00F3,
        4 => 0x0067,
        5 => 0x00B7,
        6 => 0x00BF,
        7 => 0x00E0,
        8 => 0x00FF,
        9 => 0x00F7,
        _ => 0x0000,
    }
}

struct Digits {
    mask: usize,
    num: usize,
}

impl Iterator for Digits {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            return None;
        };
        let d = self.num / self.mask % 10;
        self.mask /= 10;
        Some(d as u16)
    }
}

impl DoubleEndedIterator for Digits {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = self.num;
        self.num -= 1;
        Some(result as u16)
    }
}

#[allow(overflowing_literals)]
impl Digits {
    pub fn new(n: usize) -> Self {
        Digits {
            mask: match n {
                0..=10 => 10,
                10..=100 => 100,
                100..=1000 => 1000,
                1000..=10_000 => 10_000,
                10_000..=10_0000 => 10_0000,
                _ => 1,
            },
            num: n,
        }
    }
}

pub fn write_digit(num: i16, lcd: &mut LCD_E) {
    let iter = Digits::new(num as usize);
    let mut d: u8 = 0;
    for i in iter {
        match d {
            0 => write_dig_pos(i, 0, lcd),
            1 => write_dig_pos(i, 1, lcd),
            2 => write_dig_pos(i, 2, lcd),
            3 => write_dig_pos(i, 3, lcd),
            4 => write_dig_pos(i, 4, lcd),
            5 => write_dig_pos(i, 5, lcd),
            6 => write_dig_pos(i, 6, lcd),
            _ => return,
        }
        d += 1;
    }
}

pub fn write_string(s: &str, lcd: &mut LCD_E) {
    for i in s.char_indices() {
        match i.0 {
            0 => write_char_pos(i.1, 1, lcd),
            1 => write_char_pos(i.1, 2, lcd),
            2 => write_char_pos(i.1, 3, lcd),
            3 => write_char_pos(i.1, 4, lcd),
            4 => write_char_pos(i.1, 5, lcd),
            5 => write_char_pos(i.1, 6, lcd),
            _ => return,
        }
    }
}

pub fn write_char_pos(c: char, pos: u8, lcd: &mut LCD_E) {
    let c = lookup_char(c);
    match pos {
        1 => lcd.lcdm4w.modify(|_, w| unsafe { w.bits(c) }),
        2 => lcd.lcdm6w.modify(|_, w| unsafe { w.bits(c) }),
        3 => lcd.lcdm8w.modify(|_, w| unsafe { w.bits(c) }),
        4 => lcd.lcdm10w.modify(|_, w| unsafe { w.bits(c) }),
        5 => lcd.lcdm2w.modify(|_, w| unsafe { w.bits(c) }),
        6 => lcd.lcdm18w.modify(|_, w| unsafe { w.bits(c) }),
        _ => return,
    }
}
pub fn write_dig_pos(dig: u16, pos: u8, lcd: &mut LCD_E) {
    let d = lookup_number(dig);
    // let d = lookup_char(char::from(dig as u8));
    match pos {
        1 => lcd.lcdm4w.modify(|_, w| unsafe { w.bits(d) }),
        2 => lcd.lcdm6w.modify(|_, w| unsafe { w.bits(d) }),
        3 => lcd.lcdm8w.modify(|_, w| unsafe { w.bits(d) }),
        4 => lcd.lcdm10w.modify(|_, w| unsafe { w.bits(d) }),
        5 => lcd.lcdm2w.modify(|_, w| unsafe { w.bits(d) }),
        6 => lcd.lcdm18w.modify(|_, w| unsafe { w.bits(d) }),
        _ => return,
    }
}
