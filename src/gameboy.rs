use std::io::Error;

use crate::rom::BootRom;
use crate::mmu::MMU;
use crate::register::{Registers, Registers8, Registers16};
use crate::cpu::CPUManager;
use crate::cartridge::Cartridge;
use crate::msg::Frame;

use std::sync::mpsc::SyncSender;

pub struct Gameboy {
    pub cpu: CPUManager,
    sender: SyncSender<Box<Frame>>,
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

        let mut registers = if skip_boot {
            Registers::skip_boot()
        } else {
            Registers::new()
        };

        // registers.watcher.insert16(Registers16::PC, 0x0001);

        let cpu = CPUManager::new(
            registers,
            mmu,
        );

        Ok(Gameboy {
            cpu: cpu,
            sender: sender,
        })
    }

    pub fn next_frame(&mut self) {
        self.cpu.next_frame();
        self.sender.send(self.cpu.frame_info()).unwrap();
    }
}
