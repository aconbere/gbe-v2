use crate::tile::Tile;
use crate::device::Device;
use std::fmt;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub struct TileMap {
    pub storage: [u8; 2048]
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            storage: [0; 2048],
        }
    }
}

impl fmt::Display for TileMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..31 {
            for j in 0..31 {
                write!(f, "{:X}", self.storage[(i * 32) + j])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
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
    pub tile_set: [Tile; 384],
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
        // println!("Tile::tile_index: {} {}", i, bank);
        if bank {
            /* In bank 2 the index is treated as a signed value referenced from the top of the
             * first bank at index 256
             */
            let ii8: i8 = i as i8;
            let top: i16 = 256;
            let index = top - (ii8 as i16);

            self.tile_set[index as usize]
        } else {
            self.tile_set[i as usize]
        }
    }

    pub fn get_map(&self, i: u16, bank: bool) -> u8 {
        if i > 1023 {
            panic!("address out of range for tile map: {:X}", i);
        }

        if i == 272 {
            println!("GPU::get_map: {} {}", i, self.tile_map.get(272));
        }

        if bank {
            self.tile_map.get((i as u16) + 1023)
        } else {
            self.tile_map.get(i as u16)
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

        // println!("Tile::set tile_index {}", tile_index);
        // println!("Tile::set top_byte bottom_byte {}, {}", top_byte, bottom_byte);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_tile_map() {
        let mut t = TileMap::new();
        t.set(272, 0x19);
        assert_eq!(t.get(272), 0x19);
    }

    #[test]
    fn test_get_set_tile_map_gpu() {
        let mut g = GPU::new();
        g.tile_map.set(272, 0x19);
        assert_eq!(g.get_map(272, false), 0x19);
    }
}
