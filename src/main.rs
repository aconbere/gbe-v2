#[macro_use]
extern crate clap;

use std::sync::mpsc::sync_channel;
use std::thread;

mod sdl;
mod gameboy;
mod framebuffer;
mod palette;
mod pixel;
mod shade;
mod register;
mod bytes;
mod cpu;
mod gpu;
mod mmu;
mod tile;
mod device;
mod rom;
mod helpers;
mod cartridge;
mod msg;
mod instruction;
mod repl;

use crate::gameboy::Gameboy;

fn main() {
    let matches = clap_app!(anders_gameboy_emulator =>
        (version: "0.1")
        (author: "Anders Conbere <anders@conbere.org>")
        (about: "Emulates a gameboy V2")
        (@arg BOOT_ROM: --boot_rom +takes_value +required "The file of the boot rom to load.")
        (@arg GAME_ROM: --game_rom +takes_value +required "The file of the game rom to load.")
        (@arg SKIP_BOOT: --skip_boot "If true skips booting from the rom.")
        (@arg DEBUG: --debug "If true skips booting from the rom.")
    ).get_matches();


    let (debugger_sender, debugger_receiver) = sync_channel(0);

    if matches.is_present("DEBUG") {
        thread::spawn(|| {
            repl::start(debugger_sender);
        });
    }

    let (frame_sender, frame_receiver) = sync_channel(0);

    thread::spawn(move || {
        let mut gameboy = Gameboy::new(
            matches.value_of("BOOT_ROM").unwrap(),
            matches.value_of("GAME_ROM").unwrap(),
            matches.is_present("SKIP_BOOT"),
            frame_sender,
            debugger_receiver,
        ).unwrap();
        gameboy.start();
    });
    let mut display = sdl::SDL::new(frame_receiver).unwrap();
    display.start();
}
