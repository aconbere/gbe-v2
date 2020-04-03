use crate::device::Device;

pub struct Ram {
    storage: [u8;2048]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            storage: [0;2048],
        }
    }
}

impl Device for Ram {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}

