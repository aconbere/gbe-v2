use crate::palette::Shade;

pub type Buffer = [[Shade;160];144];

// pub type InternalBuffer = [[Shade;256];256];

pub fn zero(buffer: &mut Buffer) {
    for y in 0..143 {
        for x in 0..159 {
            buffer[y][x] = Shade::White
        }
    }
}

pub fn new() -> Buffer {
    [[Shade::White;160];144]
}
