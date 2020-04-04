use std::io::Error;
use std::io::Read;

use crate::helpers::open_file;
use crate::device::Device;

pub struct Rom {
    storage: [u8;256]
}

impl Rom {
    pub fn read(p: &str) -> Result<Rom, Error> {
        let mut bytes = [0; 256];
        let mut f = open_file(p)?;
        f.read(&mut bytes)?;

        Ok(Rom { storage: bytes })
    }
}

impl Device for Rom {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}
