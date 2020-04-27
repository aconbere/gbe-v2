use crate::register::{Registers, Registers16, IME};
use crate::mmu::MMU;
use crate::bytes;
use crate::device::lcd::Mode;
use crate::device::interrupt::Interrupt;
use crate::framebuffer;

use crate::instruction::opcode;
use crate::instruction::helper::call;


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HaltedState {
    Halted,
    HaltedNoJump,
    HaltBug,
    NoHalt
}

pub struct CPU {
    pub mmu: MMU,
    pub registers: Registers,

    pub buffer: framebuffer::Buffer,
    pub stopped: bool,
    pub halted: HaltedState,
    log: bool,
}

impl CPU {
    pub fn new(
        mmu: MMU,
        log: bool,
        skip_boot: bool
    ) -> CPU {
        let r = if skip_boot {
            Registers::skip_boot()
        } else {
            Registers::new()
        };

        CPU {
            mmu: mmu,
            registers: r,
            buffer: framebuffer::new(),
            stopped: false,
            halted: HaltedState::NoHalt,
            log: log,
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


    pub fn next_frame(&mut self) {
        loop {
            // println!("cpu: next frame instruction loop");
            match self.next_instruction() {
                Some((Mode::VBlank, Mode::OAM)) => {
                    self.mmu.interrupt_flag.vblank = true;
                    break;
                },
                Some((Mode::VRAM, Mode::HBlank)) => self.render_line(),
                Some((Mode::HBlank, Mode::VBlank)) => self.mmu.gpu.update_buffer(),
                _ => {},
            }

            /* If we triggered a stop or halt exist this loop */
            // if self.stopped || self.halted {
            //     break;
            // }
        }
    }

    pub fn get_opcode(&mut self) -> u16 {
        let opcode = self.advance_pc() as u16;

        /* the gameboy has two opcode spaces, the second space
         * is indicated by starting with the CB opcode. We store
         * the Prefixed opcodes with the byte prefix CB 
         */
        if opcode == 0x00CB {
            (self.advance_pc() as u16) | 0xCB00
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
                self.halted = HaltedState::NoHalt;
                self.registers.ime = IME::Disabled;
                call(self, 0x40);
            }
            Some(Interrupt::LCDStat) => {
                self.mmu.interrupt_flag.lcd_stat = false;
                self.halted = HaltedState::NoHalt;
                self.registers.ime = IME::Disabled;
                call(self, 0x48);
            }
            Some(Interrupt::Timer) => {
                self.mmu.interrupt_flag.timer = false;
                self.halted = HaltedState::NoHalt;
                self.registers.ime = IME::Disabled;
                call(self, 0x50);
            }
            Some(Interrupt::Serial) => {
                self.mmu.interrupt_flag.serial = false;
                self.halted = HaltedState::NoHalt;
                self.registers.ime = IME::Disabled;
                call(self, 0x58);
            }
            Some(Interrupt::Joypad) => {
                self.mmu.interrupt_flag.joypad = false;
                self.halted = HaltedState::NoHalt;
                self.registers.ime = IME::Disabled;
                call(self, 0x60);
            }
            None => {}
        }
    }

    pub fn next_instruction(&mut self) -> Option<(Mode, Mode)> {
        if self.halted == HaltedState::Halted {
            if self.registers.ime.flagged_on() {
                self.handle_interrupts();
            }
            self.advance_cycles(4);
            return None
        }

        if self.halted == HaltedState::HaltedNoJump {
            if self.interrupt_available().is_some() {
                self.halted = HaltedState::NoHalt;
            }

            self.advance_cycles(4);
            return None
        }

        if self.registers.ime.enabled () {
            self.handle_interrupts();
        }

        if self.registers.ime.queued() {
            self.registers.ime = IME::Enabled;
        }


        let opcode = self.get_opcode();
        let fetcher = opcode::Fetcher::new();
        let instruction = fetcher.fetch(opcode).unwrap();
        let result = instruction.call(self);
        self.advance_cycles(result.cycles)
    }

    pub fn advance_cycles(&mut self, cycles: u8) -> Option<(Mode, Mode)> {
        if self.mmu.timer.advance_cycles(cycles) {
            self.mmu.interrupt_flag.timer = true;
        }

        if self.halted == HaltedState::NoHalt {
            self.mmu.lcd.advance_cycles(cycles)
        } else {
            None
        }

    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn halt(&mut self) {
        if self.registers.ime.enabled() {
            self.halted = HaltedState::Halted;
        } else {
            let _if = u8::from(self.mmu.interrupt_flag);
            let _ie = u8::from(self.mmu.interrupt_enable);
            if (_if & _ie & 0x1F) == 0 {
                self.halted = HaltedState::HaltedNoJump;
            } else {
                self.halted = HaltedState::HaltBug;
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

