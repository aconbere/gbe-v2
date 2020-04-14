pub mod tile_map;
pub mod ram;
pub mod lcd;
pub mod interrupt;

pub trait Device {
    fn get(&self, a: u16) -> u8;
    fn set(&mut self, a: u16, v: u8);
}
