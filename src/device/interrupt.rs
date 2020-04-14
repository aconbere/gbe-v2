use crate::bytes;

pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

#[derive(Debug, Clone, Copy)]
pub struct InterruptFlag {
    pub vblank: bool,
    pub lcd_stat: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
}

impl InterruptFlag {
    pub fn new() -> InterruptFlag {
        InterruptFlag {
            vblank: false,
            lcd_stat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }
}

impl std::convert::From<u8> for InterruptFlag {
    fn from(byte: u8) -> Self {
        InterruptFlag {
            vblank: bytes::check_bit(byte, 0),
            lcd_stat: bytes::check_bit(byte, 1),
            timer: bytes::check_bit(byte, 2),
            serial: bytes::check_bit(byte, 3),
            joypad: bytes::check_bit(byte, 4),
        }
    }
}

impl std::convert::From<InterruptFlag> for u8 {
    fn from(p: InterruptFlag) -> Self {
        let mut u:u8 = 0x00;

        u = bytes::set_bit(u, 0, p.vblank);
        u = bytes::set_bit(u, 1, p.lcd_stat);
        u = bytes::set_bit(u, 2, p.timer);
        u = bytes::set_bit(u, 3, p.serial);
        u = bytes::set_bit(u, 4, p.joypad);

        u
    }
}
