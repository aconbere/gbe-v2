use crate::device::Device;
use crate::palette::{get_shade, Shade};
use crate::tile::Pixel;
use crate::bytes;

// 0xFF40 = control register
// 0xFF41 = status register
// 0xFF42 = scroll_y
// 0xFF43 = scroll_x
// 0xFF44 = lcd_y
// 0xFF45 = compare
// 0xFF46 = compare
// 0xFF47 = bg_palette
// 0xFF48 = object_palette_0
// 0xFF49 = object_palette_1
// 0xFF4A = window_y
// 0xFF4B = window_x

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Mode {
    // OAM Read mode
    OAM = 2,

    // VRAM Read mode
    // End of VRAM is a completed scanline
    VRAM = 3,

    // End of a scanline until the beginning of a new scanline
    // At the end of the last hblank we'll render our full frame
    HBlank = 0,

    // End of a frame, vblank lasts ~10 lines
    VBlank = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    shades: [Shade;4],
    value: u8,
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            shades: [Shade::White;4],
            value: 0,
        }
    }
    
    pub fn map(&self, px: Pixel) -> Shade {
        self.shades[px as usize]
    }

}


impl std::convert::From<u8> for Palette {
    fn from(byte: u8) -> Self {
        Palette {
            value: byte,
            shades: [
                get_shade(byte, 0),
                get_shade(byte, 1),
                get_shade(byte, 2),
                get_shade(byte, 3)
            ],
        }
    }
}

impl std::convert::From<Palette> for u8 {
    fn from(p: Palette) -> Self {
        p.value
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StatusRegister {
    pub ly_coincidence_interrupt: bool,
    pub oam_interrupt: bool,
    pub vblank_interrupt: bool,
    pub hblank_interrupt: bool,
    pub coincidence: bool,
    pub mode: Mode,
}

/* Status Register
 * Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)
 * Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)
 * Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)
 * Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)
 * Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)
 * Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)
 *           0: During H-Blank
 *           1: During V-Blank
 *           2: During Searching OAM
 *           3: During Transferring Data to LCD Driver
 */
impl StatusRegister {
    pub fn new() -> StatusRegister {
        StatusRegister {
            ly_coincidence_interrupt: false,
            oam_interrupt: false,
            vblank_interrupt: false,
            hblank_interrupt: false,
            coincidence: false,
            mode: Mode::OAM,
        }
    }
}

impl std::convert::From<u8> for StatusRegister {
    fn from(byte: u8) -> Self {
        let mode = match (bytes::check_bit(byte, 1), bytes::check_bit(byte, 0)) {
            (false, false) => Mode::HBlank,
            (false, true) => Mode::VRAM,
            (true, false) => Mode::OAM,
            (true, true) => Mode::VBlank,
        };

        StatusRegister {
            ly_coincidence_interrupt: bytes::check_bit(byte, 6),
            oam_interrupt: bytes::check_bit(byte, 5),
            vblank_interrupt: bytes::check_bit(byte, 4),
            hblank_interrupt: bytes::check_bit(byte, 3),
            coincidence: bytes::check_bit(byte, 2),
            mode: mode,
        }
    }
}

impl std::convert::From<StatusRegister> for u8 {
    fn from(r: StatusRegister) -> Self {

        let mut u = match r.mode {
            Mode::HBlank => 0b00,
            Mode::VRAM => 0b01,
            Mode::OAM => 0b10,
            Mode::VBlank => 0b11,
        };

        u = bytes::set_bit(u, 6, r.ly_coincidence_interrupt);
        u = bytes::set_bit(u, 5, r.oam_interrupt);
        u = bytes::set_bit(u, 4, r.vblank_interrupt);
        u = bytes::set_bit(u, 3, r.hblank_interrupt);
        u = bytes::set_bit(u, 2, r.coincidence);

        u
    }
}

/* Control Register
 *
 * Bit 7 - LCD Display Enable             (0=Off, 1=On)
 * Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)
 * Bit 5 - Window Display Enable          (0=Off, 1=On)
 * Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
 * Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
 * Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
 * Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)
 * Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)
 */

#[derive(Debug, Clone, Copy)]
pub struct ControlRegister {
    pub lcd_enabled: bool,
    pub tile_map: bool,
    pub window_enabled: bool,
    pub tile_data: bool,
    pub display_select: bool,
    pub sprite_size: bool,
    pub sprite_enabled: bool,
    pub window_priority: bool,
}

impl ControlRegister {
    pub fn new() -> ControlRegister {
        ControlRegister {
            lcd_enabled: false,
            tile_map: false,
            window_enabled: false,
            tile_data: false,
            display_select: false,
            sprite_size: false,
            sprite_enabled: false,
            window_priority: false,
        }
    }
}

impl std::convert::From<u8> for ControlRegister {
    fn from(byte: u8) -> Self {
        ControlRegister {
            lcd_enabled: bytes::check_bit(byte, 7),
            tile_map: bytes::check_bit(byte, 6),
            window_enabled: bytes::check_bit(byte, 5),
            tile_data: bytes::check_bit(byte, 4),
            display_select: bytes::check_bit(byte, 3),
            sprite_size: bytes::check_bit(byte, 2),
            sprite_enabled: bytes::check_bit(byte, 1),
            window_priority: bytes::check_bit(byte, 0),
        }
    }
}

impl std::convert::From<ControlRegister> for u8 {
    fn from(r: ControlRegister) -> Self {
        let mut u:u8 = 0;

        u = bytes::set_bit(u, 7, r.lcd_enabled);
        u = bytes::set_bit(u, 6, r.tile_map);
        u = bytes::set_bit(u, 5, r.window_enabled);
        u = bytes::set_bit(u, 4, r.tile_data);
        u = bytes::set_bit(u, 3, r.display_select);
        u = bytes::set_bit(u, 2, r.sprite_size);
        u = bytes::set_bit(u, 1, r.sprite_enabled);
        u = bytes::set_bit(u, 0, r.window_priority);

        u
    }
}

pub struct LCD {
    cycles: u32,
    pub lines: u8,
    mode_clock: u32,

    pub control: ControlRegister,
    pub status: StatusRegister,

    pub scroll_y: u8,
    pub scroll_x: u8,
    pub lcd_y: u8,
    pub ly_compare: u8,
    pub dma: u8,
    pub bg_palette: Palette,
    pub object_palette_0: Palette,
    pub object_palette_1: Palette,
    pub window_y: u8,
    pub window_x: u8,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            cycles: 0,
            lines: 0,
            mode_clock: 0,

            control: ControlRegister::new(),
            status: StatusRegister::new(),
            scroll_y: 0,
            scroll_x: 0,
            lcd_y: 0,
            ly_compare: 0,
            dma: 0,
            bg_palette: Palette::new(),
            object_palette_0: Palette::new(),
            object_palette_1: Palette::new(),
            window_y: 0,
            window_x: 0,
        }
    }
    

    pub fn advance_cycles(&mut self, n: u8) -> Option<Mode> {
        self.cycles = self.cycles.wrapping_add(n as u32);
        self.mode_clock = self.mode_clock.wrapping_add(n as u32);

        match self.status.mode {
            Mode::OAM => {
                if self.mode_clock >= 80 {
                    self.status.mode = Mode::VRAM;
                    Some(self.status.mode)
                } else {
                    None
                }
            }
            Mode::VRAM => {
                if self.mode_clock >= 252 {
                    // self.render_line();
                    self.status.mode = Mode::HBlank;
                    Some(self.status.mode)
                } else {
                    None
                }
            }
            Mode::HBlank => {
                if self.mode_clock >= 456 {
                    self.mode_clock -= 456;

                    self.lines += 1;

                    if self.lines == 144 {
                        self.status.mode = Mode::VBlank;
                        Some(self.status.mode)
                    } else {
                        self.status.mode = Mode::OAM;
                        Some(self.status.mode)
                    }
                } else {
                    None
                }
            }
            Mode::VBlank => {
                if self.mode_clock >= 456 {
                    self.mode_clock -= 456;
                    self.lines += 1;
                }

                if self.lines == 153 {
                    self.lines = 0;
                    self.status.mode = Mode::OAM;
                    Some(self.status.mode)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_y_offset(&self) -> u8 {
        self.lines.wrapping_add(self.scroll_y)
    }
}

impl Device for LCD {
    fn get(&self, address: u16) -> u8 {
        match address {
            0x0040 => u8::from(self.control),
            0x0041 => u8::from(self.status),
            0x0042 => self.scroll_y,
            0x0043 => self.scroll_x,
            0x0044 => self.lcd_y,
            0x0045 => self.ly_compare,
            0x0046 => self.dma,
            0x0047 => u8::from(self.bg_palette),
            0x0048 => u8::from(self.object_palette_0),
            0x0049 => u8::from(self.object_palette_1),
            0x004A => self.window_y,
            0x004B => self.window_x,
            _ => panic!("invalid lcd address: {:X}", address),
        }
    }

    fn set(&mut self, address: u16, v: u8) {
        match address {
            0x0040 => self.control = ControlRegister::from(v),
            0x0041 => self.status = StatusRegister::from(v),
            0x0042 => self.scroll_y = v,
            0x0043 => self.scroll_x = v,
            0x0044 => self.lcd_y = v,
            0x0045 => self.ly_compare = v,
            0x0046 => self.dma = v,
            0x0047 => self.bg_palette = Palette::from(v),
            0x0048 => self.object_palette_0 = Palette::from(v),
            0x0049 => self.object_palette_1 = Palette::from(v),
            0x004A => self.window_y = v,
            0x004B => self.window_x = v,
            _ => panic!("invalid lcd address: {:X}", address),
        }
    }
}
