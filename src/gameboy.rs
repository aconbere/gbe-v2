use std::io::Error;

use crate::rom::BootRom;
use crate::mmu::MMU;
use crate::cpu::CPU;
use crate::cartridge::Cartridge;
use crate::msg::{Frame, TileMap};
use crate::shade::Shade;

use std::sync::mpsc::SyncSender;

pub struct Gameboy {
    pub cpu: CPU,
    sender: SyncSender<Frame>,
}

impl Gameboy {
    pub fn new(
        boot_rom: &str,
        game_rom: &str,
        log: bool,
        skip_boot: bool,
        sender: SyncSender<Frame>,
    ) -> Result<Gameboy, Error> {
        let cartridge = Cartridge::read(game_rom)?;
        let boot_rom = BootRom::read(boot_rom)?;

        let mmu = if skip_boot {
            MMU::skip_boot(cartridge)
        } else {
            MMU::new(boot_rom, cartridge)
        };

        let cpu = CPU::new(
            mmu,
            log,
            skip_boot,
        );

        Ok(Gameboy {
            cpu: cpu,
            sender: sender,
        })
    }

    pub fn draw_tile_map(&self) -> TileMap {
        println!("draw tile map");
        TileMap {
            palette: self.cpu.mmu.lcd.bg_palette,
            pixels: self.cpu.mmu.gpu.buffer,
            scroll_x: self.cpu.mmu.lcd.scroll_x,
            scroll_y: self.cpu.mmu.lcd.scroll_y,
        }
    }

    pub fn draw_tiles(&self) -> [[Shade; 256]; 96] {
        println!("draw_tiles");
        [[Shade::White;256];96]
    }

    pub fn send_frame(&mut self) {
        let frame_info = Frame {
            main: self.cpu.buffer,
            tiles: self.draw_tiles(),
            tile_map: self.draw_tile_map(),
        };
        println!("gameboy: frame info");

        let result = self.sender.try_send(frame_info);

        println!("gameboy: send result: {:?}", result);
    }

    pub fn next_frame(&mut self) {
        println!("gameboy: next frame");
        self.cpu.next_frame();
        println!("gameboy: next frame: End");
        self.send_frame();
    }
}
