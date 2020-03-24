#[macro_use]
extern crate clap;

mod sdl;
mod gameboy;
mod framebuffer;
mod palette;

use gameboy::Gameboy;
use sdl::SDL;

fn main() {
    let matches = clap_app!(anders_gameboy_emulator =>
        (version: "0.1")
        (author: "Anders Conbere <anders@conbere.org>")
        (about: "Emulates a gameboy V2")
        (@arg BOOT_ROM: --boot_rom +takes_value +required "The file of the boot rom to load.")
        (@arg GAME_ROM: --game_rom +takes_value +required "The file of the game rom to load.")
        (@arg CONFIG: --config +takes_value "An optional configuration file to read.")
    ).get_matches();

    let params = gameboy::StartParameters::new(
        matches.value_of("BOOT_ROM").unwrap(),
        matches.value_of("GAME_ROM").unwrap(),
        matches.value_of("CONFIG"),
    ).unwrap();


    let mut gameboy = Gameboy::new(&params);
    let mut display = SDL::new().unwrap();
    display.start(&mut gameboy);
}
