use crate::tile::Tile;
use crate::device::Device;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(PartialEq)]
pub enum Mode {
    // OAM Read mode
    OAM,

    // VRAM Read mode
    // End of VRAM is a completed scanline
    VRAM,

    // End of a scanline until the beginning of a new scanline
    // At the end of the last hblank we'll render our full frame
    HBlank,

    // End of a frame, vblank lasts ~10 lines
    VBlank,
}

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}

/* VRAM layout
 * 8000-87FF: First part of tile set #1
 * 8800-8FFF: Second part of tile set #1 / First part of tile set #2
 * 9000-97FF: Second part of tile set #2
 */

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            tile_set: [Tile::zero(); 384],
        }
    }
}

impl Device for GPU {
    fn set(&mut self, address: u16, value: u8) {
        self.vram[address as usize] = value;

        /* only support the first bank of tiles for now */
        if address >= 0x1800 { return }

        /* Tile data is stored in a sequence of pairs of bytes
         * but it always starts on the even byte, this allows
         * us to figure out the exact pair of bytes that are associated
         * with this Tile and fetch them.
         */
        let normalized_index = make_even(address) as usize;

        /* A tile is found every 16 bytes */
        let tile_index = address / 16;

        /* Rows are two bytes long */
        let row_index = ((address % 16) / 2) as u8;

        let top_byte = self.vram[normalized_index];
        let bottom_byte = self.vram[normalized_index + 1];

        self.tile_set[tile_index as usize].set_row(row_index, top_byte, bottom_byte);
    }

    fn get(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }
}

fn make_even(i: u16) -> u16 {
    // 0x01 is the first byte making it 0 will make the number even
    i & !0x0001
}
