use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::io::ErrorKind;

use super::framebuffer;
use super::palette;

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
    frame_count: u32,
}

impl Gameboy {
    pub fn new(params: &StartParameters) -> Gameboy {
        Gameboy {
            frame_count: 0,
        }
    }

    pub fn next_instruction(&self, buffer: &mut framebuffer::Framebuffer) -> bool {
        buffer[1000] = palette::Shade::Black;
        buffer[1001] = palette::Shade::LightGrey;
        buffer[1002] = palette::Shade::DarkGrey;
        true
    }

    pub fn next_frame(&mut self, buffer: &mut framebuffer::Framebuffer) {
        buffer[1000] = palette::Shade::Black;
        buffer[1001] = palette::Shade::LightGrey;
        buffer[1002] = palette::Shade::DarkGrey;
        self.frame_count += 1;
    }
}
