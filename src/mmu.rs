use super::bytes;

pub struct MMU {
}

impl MMU {
    pub fn new() -> MMU {
        MMU {}
    }
    pub fn get8(&self, address: u16) -> u8 {
        0x01
    }

    pub fn get16(&self, address: u16) -> u16 {
        let ms = self.get8(address);
        let ls = self.get8(address+1);
        bytes::combine_ms_ls(ms, ls)
    }

    pub fn set8(&self, address: u16, value: u8) {
    }

    pub fn set16(&self, address: u16, value: u16) {
        let (ms, ls) = bytes::split_ms_ls(value);
        self.set8(address, ms);
        self.set8(address.wrapping_add(1), ls);
    }
}
