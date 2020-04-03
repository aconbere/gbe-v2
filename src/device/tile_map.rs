use crate::device::Device;

pub struct TileMap {
    storage: [u8;2048]
}

impl TileMap {
    pub fn new() -> TileMap {
        TileMap {
            storage: [0;2048],
        }
    }
}

impl Device for TileMap {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}
