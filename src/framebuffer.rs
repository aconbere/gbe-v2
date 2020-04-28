use crate::shade::Shade;

pub type Buffer = [[Shade;160];144];

pub fn new() -> Buffer {
    [[Shade::White;160];144]
}
