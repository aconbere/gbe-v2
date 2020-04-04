#[derive(Debug, Clone, Copy)]
pub enum Shade {
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

const MASK:u8 = 0b000000011;

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

    let shift = i * 2;

    let v = (byte & (MASK << shift)) >> shift;

    match v {
        0 => Shade::White,
        1 => Shade::LightGrey,
        2 => Shade::DarkGrey,
        3 => Shade::Black,
        _ => panic!("Invalid shade index: {:X}", i),
    }
}
