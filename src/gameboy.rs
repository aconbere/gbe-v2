use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::io::ErrorKind;

use super::framebuffer;
use super::framebuffer::Framebuffer;
use super::palette;
use super::register::{Registers, Registers16};
use super::mmu::MMU;
use super::cpu;
use super::bytes;

const FRAME_CYCLES:u32 = 70244;

type RomMemory = [u8; 256];

pub struct Config {
    debug: bool,
}

impl Config {
    pub fn new(f: &mut File) -> Config {
        Config{debug: false}
    }

    pub fn default() -> Config {
        Config{debug: false}
    }
}

pub struct BootRom {
    storage: [u8;256]
}

impl BootRom {
    pub fn new(s: [u8; 256]) -> BootRom {
        BootRom{ storage: s }
    }
}

pub struct GameRom {
    storage: [u8;256]
}

impl GameRom {
    pub fn new(s: [u8; 256]) -> GameRom {
        GameRom{ storage: s }
    }
}

pub struct StartParameters {
    boot_rom: BootRom,
    game_rom: GameRom,
    config: Config,
}


fn open_file(p: &str) -> Result<File, Error> {
    let path = Path::new(p);

    if !path.exists() {
        return Err(Error::new(ErrorKind::Other, format!("Path does not exist: {}", p)));
    }

    let f = File::open(path)?;

    Ok(f)
}

fn read_rom(p: &str) -> Result<RomMemory, Error> {
    let mut m = [0; 256];
    let mut f = open_file(p)?;
    f.read(&mut m)?;

    Ok(m)
}

impl StartParameters {
    pub fn new(
        boot_rom_param: &str,
        game_rom_param: &str,
        config_param: Option<&str>
    ) -> Result<StartParameters, Error> {
        let boot_rom = {
            let m = read_rom(boot_rom_param)?;
            BootRom::new(m)
        };

        let game_rom = {
            let m = read_rom(boot_rom_param)?;
            GameRom::new(m)
        };


        let config = match config_param {
            Some(c) => {
                let mut f = open_file(c)?;
                Config::new(&mut f)
            },
            None => Config::default()
        };

        Ok(StartParameters {
            boot_rom: boot_rom,
            game_rom: game_rom,
            config: config,
        })
    }
}

pub struct Gameboy {
    pub frame_count: u32,
    framebuffer: Framebuffer,
    framebuffer_next: Framebuffer,
    cycles: u32,
    stopped: bool,
    halted: bool,
    interupts_enabled: bool,
    pub mmu: MMU,
    pub registers: Registers,
}

impl Gameboy {
    pub fn new(params: &StartParameters) -> Gameboy {
        Gameboy {
            frame_count: 0,
            framebuffer: framebuffer::new(),
            framebuffer_next: framebuffer::new(),
            cycles: 0,
            stopped: false,
            halted: false,
            interupts_enabled: false,
            registers: Registers::new(),
            mmu: MMU::new(),
        }
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.registers.get16(Registers16::PC);
        self.registers.inc_pc();
        self.mmu.get8(pc)
    }

    pub fn fetch_opcode(&mut self) -> u16{
        let opcode = self.advance_pc() as u16;

        /* the gameboy has two opcode spaces, the second space
         * is indicated by starting with the CB opcode.
         */
        if opcode == 0x00CB {
            self.fetch_opcode() << 8
        } else {
            opcode
        }
    }

    pub fn execute_opcode(&mut self, opcode:u16) {
        let mut s = self;
        cpu::execute(&mut s, opcode);
    }

    pub fn next_instruction(&mut self) -> bool {
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);

        // If is a new frame (clock check)
        if self.cycles >= FRAME_CYCLES {
            // if we crossed 70244 we want to loop back around
            self.cycles -= FRAME_CYCLES;
            true
        } else {
            false
        }

    }

    fn swap_buffers(&mut self) {
        self.framebuffer = self.framebuffer_next;
        self.framebuffer_next = framebuffer::new();
    }

    pub fn next_frame(&mut self) {
        loop {
            if self.next_instruction() {
                break;
            }
        }
        self.frame_count += 1;

        let mut buffer = framebuffer::new();
        buffer[1000] = palette::Shade::Black;
        buffer[1001] = palette::Shade::LightGrey;
        buffer[1002] = palette::Shade::DarkGrey;
        self.framebuffer_next = buffer;
        self.swap_buffers();
    }

    pub fn get_current_frame(&self) -> Framebuffer {
        self.framebuffer
    }

    pub fn fetch_arg_8(&mut self) -> u8 {
        self.advance_pc()
    }

    pub fn fetch_arg_16(&mut self) -> u16 {
        let v1 = self.advance_pc();
        let v2 = self.advance_pc();
        bytes::combine_ms_ls(v2, v1)
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn advance_cycles(&mut self, c:u32) {
        self.cycles += c;
    }

    pub fn enable_interrupts(&mut self) {
        self.interupts_enabled = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interupts_enabled = false;
    }
}
