#[macro_use]
extern crate clap;

use std::sync::mpsc::{channel, sync_channel};
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

use gameboy::Gameboy;
use cpu::next_frame;
use register::{RPair, Registers16};

fn main() {
    let matches = clap_app!(anders_gameboy_emulator =>
        (version: "0.1")
        (author: "Anders Conbere <anders@conbere.org>")
        (about: "Emulates a gameboy V2")
        (@arg BOOT_ROM: --boot_rom +takes_value +required "The file of the boot rom to load.")
        (@arg GAME_ROM: --game_rom +takes_value +required "The file of the game rom to load.")
        (@arg LOG: --log "If true print debug output.")
        (@arg SKIP_BOOT: --skip_boot "If true skips booting from the rom.")
        (@arg CONFIG: --config +takes_value "An optional configuration file to read.")
    ).get_matches();

    let (frame_sender, frame_receiver) = sync_channel(0);
    let (output_sender, output_receiver) = channel();
    let (input_sender, input_receiver) = channel();

    thread::spawn(move || {
        let mut gameboy = Gameboy::new(
            matches.value_of("BOOT_ROM").unwrap(),
            matches.value_of("GAME_ROM").unwrap(),
            matches.is_present("SKIP_BOOT"),
        ).unwrap();

        // test to make sure the watcher operates
        gameboy.cpu.registers.watcher.set_break_point(
            RPair::R16(Registers16::PC, 0x0100)
        );

        // TODO Async has made the cpu run faster than the display
        // These need to be synced somehow
        loop {
            next_frame(
                &mut gameboy.cpu,
                &gameboy.instructions,
                &frame_sender,
                &output_sender,
                &input_receiver
            );
        }
    });

    let mut display = sdl::SDL::new(frame_receiver, output_receiver, input_sender).unwrap();
    display.start();
}
