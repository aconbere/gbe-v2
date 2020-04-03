use std::io::Error;
use std::io::Read;

use crate::helpers::open_file;
use crate::device::Device;

type RomMemory = [u8; 256];

pub struct Rom {
    storage: [u8;256]
}

impl Rom {
    pub fn new(s: [u8; 256]) -> Rom {
        Rom { storage: s }
    }

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
