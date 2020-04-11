use crate::bytes;

#[derive(Debug, Clone, Copy)]
pub struct InterruptEnable {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl InterruptEnable {
    pub fn new() -> InterruptEnable {
        InterruptEnable {
            vblank: false,
            lcd_stat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }
}

impl std::convert::From<u8> for InterruptEnable {
    fn from(byte: u8) -> Self {
        InterruptEnable {
            vblank: bytes::check_bit(byte, 0),
            lcd_stat: bytes::check_bit(byte, 1),
            timer: bytes::check_bit(byte, 2),
            serial: bytes::check_bit(byte, 3),
            joypad: bytes::check_bit(byte, 4),
        }
    }
}

impl std::convert::From<InterruptEnable> for u8 {
    fn from(p: InterruptEnable) -> Self {
        let mut u:u8 = 0x00;

        u = bytes::set_bit(u, 0, p.vblank);
        u = bytes::set_bit(u, 1, p.lcd_stat);
        u = bytes::set_bit(u, 2, p.timer);
        u = bytes::set_bit(u, 3, p.serial);
        u = bytes::set_bit(u, 4, p.joypad);

        u
    }
}

