use crate::bytes;
use crate::pixel::Pixel;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub data: [[Pixel; 8];8],
}

/*
* Tiles are 8x8 pixels and layed out where every two bits defines a pixel. They are
* 16 bytes and with every two bytes defining a row. Oddly pixels are aligned vertically
* in these two byte rows... for example
*
* 1. [0,1,0,0,1,1,1,0]
* 2. [1,0,0,0,1,0,1,1]
* 
* ...
* 
* 15. [1,0,0,0,1,0,1,1]
* 16. [1,0,0,0,1,0,1,1]
* 
* results in a row of pixels:
* 
* 1. [2,1,0,0,3,1,3.2]
* ...
* 8. [2,1,0,0,3,1,3.2]
*/
impl Tile {
    pub fn set_row(&mut self, row: u8, top_byte: u8, bottom_byte: u8) {
        for i in 0..8 {
            let byte_index = 7 - i;

            let bits = (
                bytes::check_bit(top_byte, byte_index),
                bytes::check_bit(bottom_byte, byte_index),
            );

            let p = match bits {
                (true, true) => Pixel::P3,
                (false, true) => Pixel::P2,
                (true, false) => Pixel::P1,
                (false, false) => Pixel::P0,
            };

            self.data[row as usize][i as usize] = p;
        }
    }

    pub fn zero() -> Tile {
        Tile {
            data: [[Pixel::P0;8];8],
        }
    }
}
