use crate::tile::Tile;
use crate::device::Device;
use crate::pixel::Pixel;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub struct TileMap {
    pub storage: [[u8; 32]; 64],
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            storage: [[0; 32]; 64],
        }
    }

    pub fn map(&self, y: u8, x: u8, bank: bool) -> u8 {
        if bank {
            self.storage[(y as usize) + 32][x as usize]
        } else {
            self.storage[y as usize][x as usize]
        }
    }

    pub fn set(&mut self, address: u16, value: u8) -> (u8, u8) {
        if address > 2047 {
            panic!("address out of range for tile map: {:X}", address);
        }

        let y = address / 32;
        let x = address - (y * 32);

        self.storage[y as usize][x as usize] = value;

        (x as u8, y as u8)
    }

    fn get(&self, address: u16) -> u8 {
        if address > 2047 {
            panic!("address out of range for tile map: {:X}", address);
        }

        let y = address / 32;
        let x = address - (y * 32);

        self.storage[y as usize][x as usize] 
    }
}

pub struct VRam {
    storage: [u8; VRAM_SIZE],
    pub tile_set: [Tile; 384],
}

impl VRam {
    pub fn new() -> VRam {
        VRam {
            storage: [0; VRAM_SIZE],
            tile_set: [Tile::zero(); 384],
        }
    }
}

impl Device for VRam {
    fn set(&mut self, address: u16, value: u8) { 
        self.storage[address as usize] = value;

        /* Tile data is stored in a sequence of pairs of bytes
         * but it always starts on the even byte, this allows
         * us to figure out the exact pair of bytes that are associated
         * with this Tile and fetch them.
         */
        let normalized_index = make_even(address) as usize;

        /* Tiles are two bytes */
        let top_byte = self.storage[normalized_index];
        let bottom_byte = self.storage[normalized_index + 1];

        /* A tile is found every 16 bytes */
        let tile_index = address as usize  / 16;

        /* Rows are two bytes long */
        let row_index = ((address % 16) / 2) as u8;

        self.tile_set[tile_index].set_row(row_index, top_byte, bottom_byte);
    }

    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }
}

pub struct GPU {
    pub vram: VRam,
    pub tile_map: TileMap,
    pub buffer: [[Pixel;256];512],
}

/* VRAM layout
 * 8000-87FF: First part of tile set #1
 * 8800-8FFF: Second part of tile set #1 / First part of tile set #2
 * 9000-97FF: Second part of tile set #2
 */

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: VRam::new(),
            tile_map: TileMap::new(),
            // Buffer is the background full rendered
            buffer: [[Pixel::P0;256];512],
        }
    }

    fn draw_tile(&mut self, oy: usize, ox: usize, tile: Tile) {
        for y in 0..8 as usize {
            for x in 0..8 as usize {
                let p = tile.data[y][x];
                self.buffer[oy + y][ox + x] = p;
            }
        }
    }

    pub fn update_buffer(&mut self) {
        for y in 0..32 {
            for x in 0..32 {
                let mapping = self.tile_map.map(y, x, false);
                let tile = self.vram.tile_set[mapping as usize];
                self.draw_tile(y as usize * 8, x as usize * 8, tile);
            }
        }
    }
}

impl Device for GPU {
    fn set(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x97FF => {
                self.vram.set(address - 0x8000, value);
            },
            0x9800..=0x9FFF => {
                self.tile_map.set(address - 0x9800, value);
            },
            _ => panic!("Invalid GPU Memory Range: {:X}", address),
        }

    }

    fn get(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x97FF => self.vram.get(address - 0x8000),
            0x9800..=0x9FFF => self.tile_map.get(address - 0x9800),
            _ => panic!("Invalid GPU Memory Range: {:X}", address),
        }
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
    fn test_vram_set() {
        let mut vram = VRam::new();
        /* Set the first tile in the tileset to have pixel values
         * [P3 P2 P1 P0]
         */
        vram.set(0x0000, 0b1010_0000);
        vram.set(0x0001, 0b1100_0000);

        assert_eq!(vram.tile_set[0].data[0][0], Pixel::P3);
        assert_eq!(vram.tile_set[0].data[0][1], Pixel::P2);
        assert_eq!(vram.tile_set[0].data[0][2], Pixel::P1);
        assert_eq!(vram.tile_set[0].data[0][3], Pixel::P0);
    }
}
