use crate::bytes;

#[derive(Debug, Clone, Copy)]
pub enum Pixel {
    P0,
    P1,
    P2,
    P3,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    data: [[Pixel; 8];8],
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
    pub fn new(arr: [u8;16]) -> Tile {
        Tile {
            data: arr_to_tile(arr),
        }
    }

    pub fn set_row(&mut self, index: u8, top_byte: u8, bottom_byte: u8) {
        self.data[index as usize] = bytes_to_row(top_byte, bottom_byte);
    }

    pub fn zero() -> Tile {
        Tile {
            data: [[Pixel::P0;8];8],
        }
    }
}

fn arr_to_tile(arr: [u8; 16]) -> [[Pixel;8];8] {
    let mut data = [[Pixel::P0; 8];8];

    for i in 0..=7 {
        let offset = (i * 2) as usize;

        data[i] = bytes_to_row(arr[offset], arr[offset + 1]);
    }

    data
}

pub fn bytes_to_row(top_byte: u8, bottom_byte: u8) -> [Pixel;8] {
    let mut arr = [Pixel::P0;8];

    for i in 0..=7 {
        let bits = (bytes::check_bit(top_byte, i), bytes::check_bit(bottom_byte, i));
        let p = match bits {
            (true, true) => Pixel::P3,
            (false, true) => Pixel::P2,
            (true, false) => Pixel::P1,
            (false, false) => Pixel::P0,
        };
        arr[i as usize] = p
    }

    arr
}
