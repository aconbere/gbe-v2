use std::io::Error;
use std::io::Read;
use std::io::BufReader;
use crate::device::Device;

use std::path::Path;
use std::io::ErrorKind;
use std::fs::File;

use std::io::Seek;
use std::io::SeekFrom;

pub struct Cartridge {
    storage: Vec<u8>,
    header: Header,
}

impl Cartridge {
    pub fn read(path_str: &str) -> Result<Cartridge, Error> {
        let path = Path::new(path_str);

        if !path.exists() {
            return Err(Error::new(ErrorKind::Other, format!("Path does not exist: {}", path_str)));
        }

        let mut file = File::open(path)?;
        let header = Header::from_file(&mut file)?;

        file.seek(SeekFrom::Start(0x0000))?;

        let mut reader = BufReader::with_capacity(header.capacity(), file);
        let mut bytes = Vec::new();

        reader.read_to_end(&mut bytes)?;
        Ok(Cartridge::new(bytes, header))
    }

    pub fn new(bytes: Vec<u8>, header: Header) -> Cartridge {
        Cartridge {
            storage: bytes,
            header: header
        }
    }

    pub fn zero() -> Cartridge {
        Cartridge {
            storage: Vec::new(),
            header: Header::zero(),
        }
    }
}

impl Device for Cartridge {
    fn get(&self, address: u16) -> u8 {
        self.storage[address as usize]
    }

    fn set(&mut self, address: u16, value: u8) {
        self.storage[address as usize] = value
    }
}


/* Dig into the header details more here: https://gbdev.gg8.se/wiki/articles/The_Cartridge_Header#0148_-_ROM_Size
 */

pub struct Header {
    storage: [u8; 0x4F],
    cart_type: CartridgeType,
    rom_size: RomSize,
    name: String,
}

impl Header {
    pub fn from_file(file: &mut File) -> Result<Header, Error> {
        let mut bytes = [0;0x4F];
        file.seek(SeekFrom::Start(0x100))?;
        file.read(&mut bytes)?;

        Ok(Header::new(bytes))
    }

    pub fn capacity(&self) -> usize {
        /* Early tests suggest rom_size isn't reliabl */
        /* (self.rom_size as usize) * 16000 */
        match self.cart_type {
            CartridgeType::MCB0 => 32000,
            CartridgeType::MCB1 => 64000,
            _ => panic!("invalid cart type: {:?}", self.cart_type),
        }
    }

    pub fn new(bytes: [u8; 0x4F]) -> Header { 
        let name = std::str::from_utf8(&bytes[0x34..0x43]).unwrap().to_string();

        Header {
            storage: bytes,
            cart_type: CartridgeType::from(bytes[0x47]),
            rom_size: RomSize::from(bytes[0x48]),
            name: name,
        }
    }

    pub fn zero() -> Header {
        Header::new([0; 0x4F])
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RomSize {
    S0 = 2,
    S4 = 4,
    S8 = 8,
    S16 = 16,
    S32 = 32,
    S64 = 64,
    S128 = 128,
    S256 = 256,
    S512 = 512,
    S72 = 72,
    S80 = 80,
    S96 = 96,
}

impl RomSize {
    pub fn from(byte:u8) -> RomSize {
        match byte {
            0x00 => RomSize::S0,
            0x01 => RomSize::S4,
            0x02 => RomSize::S8,
            0x03 => RomSize::S16,
            0x04 => RomSize::S32,
            0x05 => RomSize::S64,
            0x06 => RomSize::S128,
            0x07 => RomSize::S256,
            0x08 => RomSize::S512,
            0x52 => RomSize::S72,
            0x53 => RomSize::S80,
            0x54 => RomSize::S96,
            _ => panic!("invalid rom size: {}", byte),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CartridgeType {
    MCB0                       = 0x00,
    MCB1                       = 0x01,
    MCB1RAM                    = 0x02,
    MCB1RAMBattery             = 0x03,
    MCB2                       = 0x05,
    MCB2Battery                = 0x06,
    ROMRAM                     = 0x08,
    ROMRAMBattery              = 0x09,
    MMM01                      = 0x0B,
    MMM01RAM                   = 0x0C,
    MMM01RAMBattery            = 0x0D,
    MCB3TimerBattery           = 0x0F,
    MCB3TimerRamBattery        = 0x10,
    MCB3                       = 0x11,
    MCRB3RAM                   = 0x12,
    MCB3RAMBattery             = 0x13,
    MCB5                       = 0x19,
    MCB5RAM                    = 0x1A,
    MCB5RAMBattery             = 0x1B,
    MCB5Rumble                 = 0x1C,
    MCB5RumbleRAM              = 0x1D,
    MCB5RumbleRAMBattery       = 0x1E,
    MCB6                       = 0x20,
    MCB7SensorRumbleRAMBattery = 0x22,
    PocketCamera               = 0xFC,
    BandaiTamas                = 0xFD,
    HuC3                       = 0xFE,
    HuC1RAMBattery             = 0xFF,
}

impl std::convert::From<u8> for CartridgeType {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => CartridgeType::MCB0,
            0x01 => CartridgeType::MCB1,
            0x02 => CartridgeType::MCB1RAM,
            0x03 => CartridgeType::MCB1RAMBattery,
            0x05 => CartridgeType::MCB2,
            0x06 => CartridgeType::MCB2Battery,
            0x08 => CartridgeType::ROMRAM,
            0x09 => CartridgeType::ROMRAMBattery,
            0x0B => CartridgeType::MMM01,
            0x0C => CartridgeType::MMM01RAM,
            0x0D => CartridgeType::MMM01RAMBattery,
            0x0F => CartridgeType::MCB3TimerBattery,
            0x10 => CartridgeType::MCB3TimerRamBattery,
            0x11 => CartridgeType::MCB3,
            0x12 => CartridgeType::MCRB3RAM,
            0x13 => CartridgeType::MCB3RAMBattery,
            0x19 => CartridgeType::MCB5,
            0x1A => CartridgeType::MCB5RAM,
            0x1B => CartridgeType::MCB5RAMBattery,
            0x1C => CartridgeType::MCB5Rumble,
            0x1D => CartridgeType::MCB5RumbleRAM,
            0x1E => CartridgeType::MCB5RumbleRAMBattery,
            0x20 => CartridgeType::MCB6,
            0x22 => CartridgeType::MCB7SensorRumbleRAMBattery,
            0xFC => CartridgeType::PocketCamera,
            0xFD => CartridgeType::BandaiTamas,
            0xFE => CartridgeType::HuC3,
            0xFF => CartridgeType::HuC1RAMBattery,
            _    => CartridgeType::MCB0,
        }
    }
}
