use crate::pixel::Pixel;
use crate::shade::Shade;
use crate::bytes;

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

/* Shades are stored in palettes as follows
 * Bit 7-6 - Shade for Color Number 3
 * Bit 5-4 - Shade for Color Number 2
 * Bit 3-2 - Shade for Color Number 1
 * Bit 1-0 - Shade for Color Number 0
 *
 * This function takes in index (0,1,2,3) and shifts the
 * mask 0b000000011 through the byte pulling the value
 */
pub fn get_shade(byte: u8, i:u8) -> Shade {
    if i >= 4 {
        panic!("invalid shade index must be less than 4 {:X}", i);
    }

    let i = i * 2;

    let v = (
        bytes::check_bit(byte, i),
        bytes::check_bit(byte, i + 1),
    );

    match v {
        (false, false) => Shade::White,
        (false, true) => Shade::LightGrey,
        (true, false) => Shade::DarkGrey,
        (true, true)  => Shade::Black,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shade() {
        assert_eq!(get_shade(0b1100_0000, 3), Shade::Black);
        assert_eq!(get_shade(0b0011_0000, 2), Shade::Black);
        assert_eq!(get_shade(0b0000_1100, 1), Shade::Black);
        assert_eq!(get_shade(0b0000_0011, 0), Shade::Black);

        assert_eq!(get_shade(0b0000_0000, 3), Shade::White);
        assert_eq!(get_shade(0b0100_0000, 3), Shade::DarkGrey);
        assert_eq!(get_shade(0b1000_0000, 3), Shade::LightGrey);
        assert_eq!(get_shade(0b1100_0000, 3), Shade::Black);
    }
}

