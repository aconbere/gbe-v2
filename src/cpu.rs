use crate::shade::Shade;
use crate::msg::{Frame, TileMap};
use crate::register::{Registers, Registers16, IME, HaltedState};
use crate::mmu::MMU;
use crate::bytes;
use crate::device::lcd::Mode;
use crate::device::interrupt::Interrupt;
use crate::framebuffer;
use crate::tile::Tile;
use crate::palette::Palette;

use crate::instruction::{opcode, Instruction, OpResult};
use crate::instruction::helper::call;

enum CPUAction {
    DMA,
    RenderLine,
    UpdateGPUBuffer,
    Continue,
}

pub struct CPUManager {
    instructions: opcode::Fetcher,
    cpu: CPU,
}

impl CPUManager {
    pub fn new(rs: Registers, mmu: MMU) -> CPUManager {
        CPUManager {
            instructions: opcode::Fetcher::new(),
            cpu: CPU::new(rs, mmu),
        }
    }

    pub fn frame_info(&self) -> Box<Frame> {
        Box::new(Frame {
            main: self.cpu.buffer,
            tiles: self.draw_tiles(),
            tile_map: self.draw_tile_map(),
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

    pub fn next_frame(&mut self) {
        loop {
            let action = self.next_instruction();

            match action {
                // We've finished VBlank and have moved to OAM
                // Now is the time to access DMA
                // Halt the loop and start over
                CPUAction::DMA => {
                    self.cpu.mmu.interrupt_flag.vblank = true;
                    break;
                },
                CPUAction::RenderLine => { self.cpu.render_line(); },
                // GPU is ready to have the frame buffer updated
                CPUAction::UpdateGPUBuffer => { self.cpu.mmu.gpu.update_buffer(); },

                // In all other cases we just continue looping
                CPUAction::Continue => {},
            }
        }
    }

    fn next_instruction(&mut self) -> CPUAction {
        advance_cycles(&mut self.cpu, &self.instructions)
    }
}

fn get_instruction<'a>(instructions: &'a opcode::Fetcher, opcode: u16) -> &'a Instruction {
    instructions.fetch(opcode).unwrap()
}

fn advance_cycles(cpu: &mut CPU, instructions: &opcode::Fetcher) -> CPUAction {
    match cpu.registers.halted {
        HaltedState::None => {
            if cpu.registers.ime.enabled () {
                cpu.handle_interrupts();
            }

            if cpu.registers.ime.queued() {
                cpu.registers.ime = IME::Enabled;
            }

            let opcode = cpu.get_opcode();
            let instruction = get_instruction(instructions, opcode);

            let args = cpu.get_arguments(instruction);
            let result = instruction.call(cpu, args);

            match cpu.mmu.lcd.advance_cycles(result.cycles) {
                Some((Mode::VBlank, Mode::OAM)) => CPUAction::DMA,
                Some((Mode::VRAM, Mode::HBlank)) => CPUAction::RenderLine,
                Some((Mode::HBlank, Mode::VBlank)) => CPUAction::UpdateGPUBuffer,
                _ => CPUAction::Continue,
            }
        },
        HaltedState::Halted => {
            if cpu.registers.ime.flagged_on() {
                cpu.handle_interrupts();
            }
            cpu.advance_timer(4);
            CPUAction::Continue
        },
        HaltedState::HaltedNoJump => {
            if cpu.interrupt_available().is_some() {
                cpu.registers.halted = HaltedState::None;
            }

            cpu.advance_timer(4);
            return CPUAction::Continue
        }
        // halt bug unaccounted for
        HaltedState::HaltBug => {
            return CPUAction::Continue
        }
    }
}

pub struct CPU {
    pub mmu: MMU,
    pub registers: Registers,

    pub buffer: framebuffer::Buffer,
}

impl CPU {
    pub fn new(
        registers: Registers,
        mmu: MMU,
    ) -> CPU {

        CPU {
            mmu: mmu,
            registers: registers,
            buffer: framebuffer::new(),
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> OpResult {
        let args = self.get_arguments(instruction);
        instruction.call(self, args)
    }

    fn get_arguments(&mut self, instruction: &Instruction) -> u16 {
        match instruction.args {
            0 => 0,
            1 => self.fetch_arg_8() as u16,
            2 => self.fetch_arg_16(),
            _ => panic!("Args can be 0,1,2"),
        }
    }

    fn render_line(&mut self) {
        /* Where are we in the lcd screen */
        let y = self.mmu.lcd.lines as usize;

        /* y offset tells us which row in the background buffer we're on.*/
        let bg_y = y + self.mmu.lcd.scroll_y as usize;

        /* scroll x tells us which column in the background buffer we're on */
        let bg_x = self.mmu.lcd.scroll_x as usize;

        for x in 0..160 as usize {
            let p = self.mmu.gpu.buffer[bg_y][bg_x + x as usize];
            self.buffer[y][x] = self.mmu.lcd.bg_palette.map(p);
        }
    }

    pub fn get_opcode(&mut self) -> u16 {
        let opcode = self.advance_pc() as u16;

        /* the gameboy has two opcode spaces, the second space
         * is indicated by starting with the CB opcode. We store
         * the Prefixed opcodes with the byte prefix CB 
         */
        if opcode == 0x00CB {
            (self.advance_pc() as u16) | 0x0100
        } else {
            opcode
        }
    }

    pub fn push_pc(&mut self, address: u16, value: u8) {
        self.registers.set16(Registers16::PC, address);
        self.mmu.set(address, value);
    }

    fn interrupt_available(&self) -> Option<Interrupt> {
        let _if = self.mmu.interrupt_flag;
        let _ie = self.mmu.interrupt_enable;

        if _if.vblank && _ie.vblank {
            Some(Interrupt::VBlank)
        } else if _if.lcd_stat && _ie.lcd_stat {
            Some(Interrupt::LCDStat)
        } else if _if.timer && _ie.timer {
            Some(Interrupt::Timer)
        } else if _if.serial && _ie.serial {
            Some(Interrupt::Serial)
        } else if _if.joypad && _ie.joypad {
            Some(Interrupt::Joypad)
        } else {
            None
        }
    }

    fn handle_interrupts(&mut self) {
        match self.interrupt_available() {
            Some(Interrupt::VBlank) => {
                self.mmu.interrupt_flag.vblank = false;
                self.registers.halted = HaltedState::None;
                self.registers.ime = IME::Disabled;
                call(self, 0x40);
            }
            Some(Interrupt::LCDStat) => {
                self.mmu.interrupt_flag.lcd_stat = false;
                self.registers.halted = HaltedState::None;
                self.registers.ime = IME::Disabled;
                call(self, 0x48);
            }
            Some(Interrupt::Timer) => {
                self.mmu.interrupt_flag.timer = false;
                self.registers.halted = HaltedState::None;
                self.registers.ime = IME::Disabled;
                call(self, 0x50);
            }
            Some(Interrupt::Serial) => {
                self.mmu.interrupt_flag.serial = false;
                self.registers.halted = HaltedState::None;
                self.registers.ime = IME::Disabled;
                call(self, 0x58);
            }
            Some(Interrupt::Joypad) => {
                self.mmu.interrupt_flag.joypad = false;
                self.registers.halted = HaltedState::None;
                self.registers.ime = IME::Disabled;
                call(self, 0x60);
            }
            None => {}
        }
    }

    fn advance_timer(&mut self, cycles: u8) {
        if self.mmu.timer.advance_cycles(cycles) {
            self.mmu.interrupt_flag.timer = true;
        }
    }


    pub fn stop(&mut self) {
        self.registers.stopped = true;
    }

    pub fn halt(&mut self) {
        if self.registers.ime.enabled() {
            self.registers.halted = HaltedState::Halted;
        } else {
            let _if = u8::from(self.mmu.interrupt_flag);
            let _ie = u8::from(self.mmu.interrupt_enable);
            if (_if & _ie & 0x1F) == 0 {
                self.registers.halted = HaltedState::HaltedNoJump;
            } else {
                self.registers.halted = HaltedState::HaltBug;
            }

        }
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.registers.get16(Registers16::PC);
        self.registers.inc16(Registers16::PC);
        self.mmu.get(pc)
    }

    pub fn fetch_arg_8(&mut self) -> u8 {
        self.advance_pc()
    }

    pub fn fetch_arg_16(&mut self) -> u16 {
        let v1 = self.advance_pc();
        let v2 = self.advance_pc();
        bytes::combine_ms_ls(v2, v1)
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
