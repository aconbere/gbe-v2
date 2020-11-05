use std::io::Error;

use crate::rom::BootRom;
use crate::mmu::MMU;
use crate::register::Registers; 
use crate::cpu::{next_frame, CPU};
use crate::cartridge::Cartridge;
use crate::msg::Frame;
use crate::instruction::opcode;

use std::sync::mpsc::SyncSender;

pub struct Gameboy {
    sender: SyncSender<Box<Frame>>,
    cpu: CPU,
    instructions: opcode::Fetcher,
}

impl Gameboy {
    pub fn new(
        boot_rom: &str,
        game_rom: &str,
        skip_boot: bool,
        sender: SyncSender<Box<Frame>>,
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
            sender: sender,
            instructions: instructions,
        })
    }

    pub fn next_frame(&mut self) {
        let frame = next_frame(&mut self.cpu, &self.instructions);
        self.sender.send(frame).unwrap();
    }
}
