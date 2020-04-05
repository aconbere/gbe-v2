use crate::bytes;
use crate::gpu::GPU;
use crate::device::Device;
use crate::device::ram::Ram;
use crate::device::lcd::LCD;
use crate::rom::Rom;

enum DeviceRef {
    BootRom,
    GameRom,
    VRam,
    TileMap,
    InterruptTable,
    CartridgeHeader,
    IORegisters,
    ZeroPage,
}

pub struct MMU {
    boot_rom: Rom,
    cartridge: Rom,
    io: Ram,
    zero_page: Ram,
    pub lcd: LCD,
    pub gpu: GPU,
    booted: bool,
}

impl MMU {
    pub fn new(boot_rom: Rom, game_rom: Rom) -> MMU {
        MMU {
            boot_rom: boot_rom,
            cartridge: game_rom,
            io: Ram::new(),
            zero_page: Ram::new(),
            lcd: LCD::new(),
            gpu: GPU::new(),
            booted: false,
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        match self.get_device(address) {
            (_, DeviceRef::VRam) => self.gpu.get(address),
            (start, DeviceRef::BootRom) => self.boot_rom.get(address - start),
            (start, DeviceRef::GameRom) => self.cartridge.get(address - start),
            (_start, DeviceRef::TileMap) => self.gpu.get(address),
            (start, DeviceRef::IORegisters) => {
                if address == 0xFF40 {
                    self.lcd.get(address - start)
                } else {
                    self.io.get(address - start)
                }
            },
            (start, DeviceRef::CartridgeHeader) => self.cartridge.get(address - start),
            (start, DeviceRef::ZeroPage) => self.zero_page.get(address - start),
            _ => panic!("Memory Not implemented: {:X}", address),
        }
    }

    pub fn get16(&self, address: u16) -> u16 {
        let ms = self.get(address);
        let ls = self.get(address+1);
        bytes::combine_ms_ls(ms, ls)
    }

    pub fn set(&mut self, address: u16, value: u8) {
        match self.get_device(address) {
            (_start, DeviceRef::VRam) => self.gpu.set(address, value),
            (start, DeviceRef::BootRom) => self.boot_rom.set(address - start, value),
            (start, DeviceRef::GameRom) => self.cartridge.set(address - start, value),
            (_start, DeviceRef::TileMap) => self.gpu.set(address, value),
            (start, DeviceRef::IORegisters) => {
                match address {
                    0xFF40..=0xFF4B => self.lcd.set(address - start, value),
                    _ => self.io.set(address - start, value),
                }
            }
            (start, DeviceRef::CartridgeHeader) => self.cartridge.set(address - start, value),
            (start, DeviceRef::ZeroPage) => self.zero_page.set(address - start, value),
            _ => panic!("Memory Not implemented: {:X}", address),
        }
    }

    pub fn set16(&mut self, address: u16, value: u16) {
        let (ms, ls) = bytes::split_ms_ls(value);
        self.set(address, ms);
        self.set(address.wrapping_add(1), ls);
    }

    fn get_device(&self, address: u16) -> (u16, DeviceRef) {
        match address {
            0x0000..=0x00FF => {
                if self.booted {
                    (0x0000, DeviceRef::InterruptTable)
                } else {
                    (0x0000, DeviceRef::BootRom)
                }
            },
            0x0100..=0x014F => (0x0100, DeviceRef::CartridgeHeader),
            0x0150..=0x3FFF => (0x0150, DeviceRef::GameRom),
            0x8000..=0x97FF => (0x8000, DeviceRef::VRam),
            0x9800..=0x9FFF => (0x9800, DeviceRef::TileMap),
            0xFF00..=0xFF7F => (0xFF00, DeviceRef::IORegisters),
            0xFF80..=0xFFFE => (0xFF80, DeviceRef::ZeroPage),
            _ =>  panic!("unimplemented memory location: {:X}", address)
        }
    }
}

/*
        0x00FF...0x014F => Kind::CartridgeHeader,
        0x014F...0x3FFF => Kind::CartridgeROMBank0,
        0x3FFF...0x7FFF => Kind::CartridgeROMBank1,

        // video ram
        // Because TileData is actually segmented into three sections with splits in between
        // the two sections this probably needs to be a single device.
        0x8000...0x8FFF => Kind::TileData1,
        0x8800...0x97FF => Kind::TileData2,

        0x9800...0x9BFF => Kind::TileMap1,
        0x9C00...0x9FFF => Kind::TileMap2,

        0xA000...0xBFFF => Kind::CartridgeRAM,

        // Internal Ram
        0xC000...0xCFFF => Kind::InternalRAMBank0,
        0xD000...0xDFFF => Kind::InternalRAMBank1,

        0xE000...0xFDFF => Kind::EchoRAM,
        0xFE00...0xFE9F => Kind::ObjectAttributeMemory,
        0xFEA0...0xFEFF => Kind::UnusableMemory,
        0xFF00...0xFF7F => Kind::HardwareIORegisters,
        0xFF80...0xFFFE => Kind::ZeroPage,
        0xFFFF...0xFFFF => Kind::InterruptEnableFlag,
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_tile_map() {
        let mut m = MMU::new(Rom::zero(), Rom::zero());
        let a = 0x9800 + 272;
        m.set(a, 0x19);
        assert_eq!(m.get(a), 0x19);
        assert_eq!(m.gpu.tile_map.get(272), 0x19);
    }
}
