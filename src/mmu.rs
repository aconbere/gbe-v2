use crate::bytes;
use crate::gpu::GPU;
use crate::cartridge::Cartridge;
use crate::device::Device;
use crate::device::ram::{Ram2k, Ram8k, HighRam};
use crate::device::lcd::LCD;
use crate::device::interrupt::InterruptFlag;
use crate::rom::BootRom;

#[derive(Debug, Clone, Copy)]
pub enum Frequency {
    F1024 = 1024,
    F16   = 16,
    F64   = 64,
    F256  = 256,
}

#[derive(Debug, Clone, Copy)]
pub struct TimerControl {
    enabled: bool,
    frequency: Frequency,
}

impl TimerControl {
    pub fn new() -> TimerControl {
        TimerControl {
            enabled: false,
            frequency: Frequency::F1024,
        }
    }
}

impl std::convert::From<u8> for TimerControl {
    fn from(byte: u8) -> Self {
        let f = match (bytes::check_bit(byte, 0), bytes::check_bit(byte, 1)) {
            (false, false) => Frequency::F1024,
            (false, true)  => Frequency::F16,
            (true, false)  => Frequency::F64,
            (true, true)   => Frequency::F256,
        };

        TimerControl {
            enabled: bytes::check_bit(byte, 2),
            frequency: f,
        }
    }
}

impl std::convert::From<TimerControl> for u8 {
    fn from(t: TimerControl) -> Self {
        let mut u:u8 = 0;

        u = match t.frequency {
            Frequency::F1024 => u,
            Frequency::F16 => u | 0b0000_00001,
            Frequency::F64 => u | 0b0000_00010,
            Frequency::F256 => u | 0b0000_00011,
        };

        u = bytes::set_bit(u, 2, t.enabled);

        u
    }
}

pub struct Timer {
    pub clock: u16,
    pub tma: u8,
    pub tima: u8,
    pub tac: TimerControl,

    pub tima_clock: u16,
}

impl Timer {
    pub fn advance_cycles(&mut self, n: u8) -> bool {
        self.clock = self.clock.wrapping_add(n as u16);

        if self.tac.enabled {
            self.tima_clock = self.clock.wrapping_add(n as u16);

            if self.tima_clock >= self.tac.frequency as u16 {
                let (v, overflow) = self.tima.overflowing_add(1);

                if overflow {
                    self.tima = self.tma;
                } else {
                    self.tima = v;
                }

                self.tima_clock = 0;

                return overflow
            }
        }
        false
    }

    pub fn get_div(&self, ) -> u8 {
        (self.clock >> 8) as u8
    }

    pub fn new() -> Timer {
        Timer {
            clock: 0,
            tma: 0,
            tima: 0,
            tac: TimerControl::new(),
            tima_clock: 0,
        }
    }
}

enum DeviceRef {
    BootRom,
    Cartridge,
    CartridgeRam,

    VRam,
    Ram,

    SpriteTable,
    Unused,
    IORegisters,
    HighRam,
    InterruptEnable,
}

pub struct MMU {
    boot_rom: BootRom,
    cartridge: Cartridge,
    io: Ram2k,
    ram: Ram8k,
    high_ram: HighRam,

    pub interrupt_enable: InterruptFlag,
    pub interrupt_flag: InterruptFlag,

    pub lcd: LCD,
    pub gpu: GPU,

    pub timer: Timer,

    booted: bool,
}

impl MMU {
    pub fn new(boot_rom: BootRom, cartridge: Cartridge) -> MMU {
        MMU {
            boot_rom: boot_rom,
            cartridge: cartridge,
            io: Ram2k::new(),
            ram: Ram8k::new(),
            high_ram: HighRam::new(),
            interrupt_enable: InterruptFlag::new(),
            interrupt_flag: InterruptFlag::new(),

            lcd: LCD::new(),
            gpu: GPU::new(),

            timer: Timer::new(),

            booted: false,
        }
    }

    pub fn skip_boot(cartridge: Cartridge) -> MMU {
        let mut mmu = MMU::new(BootRom::zero(), cartridge);
        mmu.set(0xFF05, 0x00);
        mmu.set(0xFF06, 0x00);
        mmu.set(0xFF07, 0x00);
        mmu.set(0xFF10, 0x80);
        mmu.set(0xFF11, 0xBF);
        mmu.set(0xFF12, 0xF3);
        mmu.set(0xFF14, 0xBF);
        mmu.set(0xFF16, 0x3F);
        mmu.set(0xFF17, 0x00);
        mmu.set(0xFF19, 0xBF);
        mmu.set(0xFF1A, 0x7F);
        mmu.set(0xFF1B, 0xFF);
        mmu.set(0xFF1C, 0x9F);
        mmu.set(0xFF1E, 0xBF);
        mmu.set(0xFF20, 0xFF);
        mmu.set(0xFF21, 0x00);
        mmu.set(0xFF22, 0x00);
        mmu.set(0xFF23, 0xBF);
        mmu.set(0xFF24, 0x77);
        mmu.set(0xFF25, 0xF3);
        mmu.set(0xFF26, 0xF1);
        mmu.set(0xFF40, 0x91);
        mmu.set(0xFF42, 0x00);
        mmu.set(0xFF43, 0x00);
        mmu.set(0xFF45, 0x00);
        mmu.set(0xFF47, 0xFC);
        mmu.set(0xFF48, 0xFF);
        mmu.set(0xFF49, 0xFF);
        mmu.set(0xFF4A, 0x00);
        mmu.set(0xFF4B, 0x00);
        mmu.set(0xFFFF, 0x00);
        mmu
    }

    pub fn get(&self, address: u16) -> u8 {
        match self.get_device(address) {
            (start, DeviceRef::BootRom) => self.boot_rom.get(address - start),
            (_, DeviceRef::Cartridge) => self.cartridge.get(address),
            (start, DeviceRef::CartridgeRam) => self.cartridge.ram.get(address - start),
            (_, DeviceRef::VRam) => self.gpu.get(address),
            (start, DeviceRef::Ram) => self.ram.get(address - start),
            (_, DeviceRef::Unused) => 0x00,
            (start, DeviceRef::IORegisters) => {
                match address {
                    0xFF04 => self.timer.get_div(),
                    0xFF05 => self.timer.tima,
                    0xFF06 => self.timer.tma,
                    0xFF07 => u8::from(self.timer.tac),
                    0xFF0F => u8::from(self.interrupt_flag),
                    0xFF40..=0xFF4B => self.lcd.get(address - start),
                    _ => self.io.get(address - start)
                }
            },
            (start, DeviceRef::HighRam) => self.high_ram.get(address - start),
            (_, DeviceRef::InterruptEnable) => u8::from(self.interrupt_enable),
            _ => panic!("Get Memory Not implemented: {:X}", address),
        }
    }

    pub fn get16(&self, address: u16) -> u16 {
        let ls = self.get(address);
        let ms = self.get(address+1);
        bytes::combine_ms_ls(ms, ls)
    }

    pub fn set(&mut self, address: u16, value: u8) {
        match self.get_device(address) {
            (_, DeviceRef::BootRom) => panic!("BootRom is read only: {:X}", address),
            (_, DeviceRef::Cartridge) => self.cartridge.set(address, value),
            (start, DeviceRef::CartridgeRam) => self.cartridge.ram.set(address - start, value),
            (_, DeviceRef::VRam) => self.gpu.set(address, value),
            (start, DeviceRef::Ram) => self.ram.set(address - start, value),
            (_, DeviceRef::Unused) => {},
            (start, DeviceRef::IORegisters) => {
                match address {
                    0xFF04 => self.timer.clock = 0,
                    0xFF05 => {
                        self.timer.tima = value;
                    },
                    0xFF06 => {
                        self.timer.tma = value;
                    },
                    0xFF07 => {
                        self.timer.tac = TimerControl::from(value);
                    },
                    0xFF0F => {
                        self.interrupt_flag = InterruptFlag::from(value);
                    },
                    0xFF40..=0xFF4B => self.lcd.set(address - start, value),
                    0xFF50 => {
                        if value == 1 {
                            self.booted = true;
                        }
                    }
                    _ => self.io.set(address - start, value),
                }
            }
            (start, DeviceRef::HighRam) => self.high_ram.set(address - start, value),
            (_start, DeviceRef::InterruptEnable) => {
                self.interrupt_enable = InterruptFlag::from(value)
            }
            _ => panic!("Set Memory Not implemented: {:X}", address),
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
                    (0x0000, DeviceRef::Cartridge)
                } else {
                    (0x0000, DeviceRef::BootRom)
                }
            },
            0x0100..=0x7FFF => (0x0150, DeviceRef::Cartridge),
            0x8000..=0x9FFF => (0x8000, DeviceRef::VRam),
            0xA000..=0xBFFF => (0xA000, DeviceRef::CartridgeRam),
            0xC000..=0xE000 => (0xC000, DeviceRef::Ram),
            0xFE00..=0xFE9F => (0xFE00, DeviceRef::SpriteTable),
            0xFEA0..=0xFEFF => (0xFEA0, DeviceRef::Unused),
            0xFF00..=0xFF7F => (0xFF00, DeviceRef::IORegisters),
            0xFF80..=0xFFFE => (0xFF80, DeviceRef::HighRam),
            0xFFFF          => (0xFFFF, DeviceRef::InterruptEnable),
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
        0xFF80...0xFFFE => Kind::HighRam,
        0xFFFF...0xFFFF => Kind::InterruptEnableFlag,
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_tile_map() {
        let mut m = MMU::new(BootRom::zero(), Cartridge::zero());
        let a = 0x9800 + 272;
        m.set(a, 0x19);
        assert_eq!(m.get(a), 0x19);
    }
}
