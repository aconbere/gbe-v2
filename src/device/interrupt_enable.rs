use crate::device::Device;

pub struct InterruptEnable {
}

impl InterruptEnable {
    pub fn new() -> InterruptEnable {
        InterruptEnable {
        }
    }
}

impl Device for InterruptEnable {
    fn get(&self, address: u16) -> u8 {
        return 0x00
    }

    fn set(&mut self, address: u16, value: u8) {
    }
}
