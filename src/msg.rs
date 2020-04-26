use crate::shade::Shade;
use crate::pixel::Pixel;
use crate::palette::Palette;

pub struct TileMap {
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub palette: Palette,
    pub pixels: [[Pixel; 256]; 512],
}

impl TileMap {
    pub fn zero() -> TileMap {
        TileMap {
            scroll_x: 0,
            scroll_y: 0,
            palette: Palette::new(),
            pixels: [[Pixel::P0; 256]; 512],
        }
    }
}

pub struct Frame {
    pub main: [[Shade;160];144],
    pub tiles: [[Shade; 256]; 96],
    pub tile_map: TileMap,
}

impl Frame {
    pub fn zero() -> Frame {
        Frame {
            main: [[Shade::White;160];144],
            tiles: [[Shade::White;256];96],
            tile_map: TileMap::zero(),
        }
    }
}
