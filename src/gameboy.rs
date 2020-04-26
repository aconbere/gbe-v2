use std::io::Error;

use crate::rom::BootRom;
use crate::mmu::MMU;
use crate::cpu::CPU;
use crate::cartridge::Cartridge;
use crate::msg::{Frame, TileMap};
use crate::shade::Shade;
use crate::tile::Tile;
use crate::palette::Palette;

use std::sync::mpsc::SyncSender;

pub struct Gameboy {
    pub cpu: CPU,
    sender: SyncSender<Box<Frame>>,
}

impl Gameboy {
    pub fn new(
        boot_rom: &str,
        game_rom: &str,
        log: bool,
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
        TileMap {
            palette: self.cpu.mmu.lcd.bg_palette,
            pixels: self.cpu.mmu.gpu.buffer,
            scroll_x: self.cpu.mmu.lcd.scroll_x,
            scroll_y: self.cpu.mmu.lcd.scroll_y,
        }
    }

    fn draw_tiles(&self) -> [[Shade;256];96] {
        let mut buffer = [[Shade::White;256];96];

        // 12 rows of tiles
        for iy in 0..12 {
            // read across for 32 tiles per row (256 pixels)
            for ix in 0..32 {
                let tile_index = (iy * 32) + ix;
                let tile = self.cpu.mmu.gpu.vram.tile_set[tile_index];
                draw_tile(
                    &mut buffer,
                    ix * 8,
                    iy * 8,
                    tile,
                    self.cpu.mmu.lcd.bg_palette,
                );
            }
        }
        buffer
    }

    pub fn send_frame(&mut self) {
        let frame_info = Box::new(Frame {
            main: self.cpu.buffer,
            tiles: self.draw_tiles(),
            tile_map: self.draw_tile_map(),
        });

        self.sender.send(frame_info).unwrap();
    }

    pub fn next_frame(&mut self) {
        self.cpu.next_frame();
        self.send_frame();
    }
}

fn draw_tile(buffer: &mut [[Shade;256];96], origin_x: usize, origin_y: usize, tile: Tile, palette: Palette) {
    for y in 0..8 as usize {
        for x in 0..8 as usize {
            let pixel = tile.data[y][x];
            let shade = palette.map(pixel);
            buffer[origin_y + y][origin_x + x] = shade;
        }
    }
}
