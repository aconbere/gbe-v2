use std::io::Error;

use crate::rom::{BootRom, GameRom};
use crate::mmu::MMU;
use crate::cpu::CPU;
use crate::sdl::SDL;

pub struct Gameboy {
    pub cpu: CPU,
}

impl Gameboy {
    pub fn new(boot_rom: &str, game_rom: &str) -> Result<Gameboy, Error> {
        let game_rom = GameRom::read(game_rom)?;
        let boot_rom = BootRom::read(boot_rom)?;

        let cpu = CPU::new(
            MMU::new(boot_rom, game_rom)
        );

        Ok(Gameboy { cpu: cpu, })
    }

    pub fn start_sdl(&mut self) {
        let mut sdl = SDL::new().unwrap();
        sdl.start(&mut self.cpu);
    }
}
