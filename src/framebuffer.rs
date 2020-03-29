use super::palette;

pub type Framebuffer = [palette::Shade; 23040];

pub fn new() -> Framebuffer {
    [palette::Shade::White; 23040]
}
