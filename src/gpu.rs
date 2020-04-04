use crate::tile::Tile;
use crate::device::Device;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub struct TileMap {
    storage: [u8; 2048]
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            storage: [0; 2048],
        }
    }
}

impl Device for TileMap {
    fn get(&self, address: u16) -> u8 {
        if address > 2047 {
            panic!("address out of range for tile map: {:X}", address);
        }
        self.storage[address as usize] 
    }

    fn set(&mut self, address: u16, value: u8) {
        if address > 2047 {
            panic!("address out of range for tile map: {:X}", address);
        }
        self.storage[address as usize] = value;
    }
}

pub struct GPU {
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
    //tile_map: [u8; 2048],
    pub tile_map: TileMap,
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
            tile_map: TileMap::new(),
        }
    }

    pub fn get_tile(&self, i: u8, bank: bool) -> Tile {
        if bank {
            self.tile_set[i as usize]
        } else {
            /* In bank 2 the index is treated as a signed value referenced from the top of the
             * first bank at index 256
             */
            let ii8: i8 = i as i8;
            let top: i16 = 256;
            let index = top - (ii8 as i16);

            self.tile_set[index as usize]
        }
    }

    pub fn get_map(&self, i: u16, bank: bool) -> u8 {
        if i > 1023 {
            panic!("address out of range for tile map: {:X}", i);
        }

        if bank {
            self.tile_map.get(i as u16)
        } else {
            self.tile_map.get((i as u16) + 1023)
        }
    }
}

impl Device for GPU {
    fn set(&mut self, address: u16, value: u8) {
        self.vram[address as usize] = value;

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
