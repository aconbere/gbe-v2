use std::io::Error;
use std::io::Read;

use crate::helpers::open_file;
use crate::device::Device;

pub struct BootRom {
    storage: [u8;256]
}

impl BootRom {
    pub fn read(p: &str) -> Result<BootRom, Error> {
        let mut bytes = [0; 256];
        let mut f = open_file(p)?;
        f.read(&mut bytes)?;

        Ok(BootRom { storage: bytes })
    }
}


impl Device for BootRom {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}

pub struct GameRom {
    storage: [u8; 32767]
}

impl GameRom {
    pub fn read(p: &str) -> Result<GameRom, Error> {
        let mut bytes = [0; 32767];
        let mut f = open_file(p)?;
        f.read(&mut bytes)?;

        Ok(GameRom { storage: bytes })
    }
}


impl Device for GameRom {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value;
    }
}
