use std::io::Error;

use crate::rom::BootRom;
use crate::mmu::MMU;
use crate::register::Registers; 
use crate::cpu::CPU;
use crate::cartridge::Cartridge;
use crate::instruction::opcode;

pub struct Gameboy {
    pub cpu: CPU,
    pub instructions: opcode::Fetcher,
}

impl Gameboy {
    pub fn new(
        boot_rom: &str,
        game_rom: &str,
        skip_boot: bool,
    ) -> Result<Gameboy, Error> {
        let cartridge = Cartridge::read(game_rom)?;
        let boot_rom = BootRom::read(boot_rom)?;

        let mmu = if skip_boot {
            MMU::skip_boot(cartridge)
        } else {
            MMU::new(boot_rom, cartridge)

        };

        let registers = if skip_boot {
            Registers::skip_boot()
        } else {
            Registers::new()
        };

        let instructions = opcode::Fetcher::new();
        let cpu = CPU::new(registers, mmu);

        Ok(Gameboy {
            cpu: cpu,
            instructions: instructions,
        })
    }
}
