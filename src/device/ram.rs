use crate::device::Device;

pub struct Ram2k {
    storage: [u8;2048]
}

impl Ram2k {
    pub fn new() -> Ram2k {
        Ram2k {
            storage: [0;2048],
        }
    }
}

impl Device for Ram2k {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}


pub struct Ram8k {
    storage: [u8; 8192],
}

impl Ram8k {
    pub fn new() -> Ram8k {
        Ram8k {
            storage: [0;8192],
        }
    }
}

impl Device for Ram8k {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}

pub struct HighRam {
    storage: [u8; 127],
}

impl HighRam {
    pub fn new() -> HighRam {
        HighRam {
            storage: [0;127],
        }
    }
}

impl Device for HighRam {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}
