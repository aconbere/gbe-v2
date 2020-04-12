use crate::register::{Registers, Registers16, Registers8};
use crate::mmu::MMU;
use crate::bytes;
use crate::device::lcd::Mode;
use crate::framebuffer;

mod instructions;

use instructions::{JumpFlag, RstFlag, _call};

pub struct CPU {
    pub mmu: MMU,
    registers: Registers,

    pub buffer: framebuffer::Buffer,
    pub stopped: bool,
    pub halted: bool,
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        CPU {
            mmu: mmu,
            registers: Registers::new(),
            buffer: framebuffer::new(),
            stopped: false,
            halted: false,
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
            match self.next_instruction() {
                Some((Mode::VBlank, Mode::OAM)) => {
                    self.mmu.interrupt_flag.vblank = true;
                    break;
                },
                Some((Mode::VRAM, Mode::HBlank)) => self.render_line(),
                Some((Mode::HBlank, Mode::VBlank)) => self.mmu.gpu.update_buffer(),
                _ => {},
            }

            if self.stopped || self.halted {
                break;
            }
        }
    }

    pub fn fetch_opcode(&mut self) -> u16{
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

    fn handle_interrupts(&mut self) {
        let ire = self.mmu.interrupt_enable;
        let mut irf = self.mmu.interrupt_flag;

        if irf.vblank && ire.vblank {
            println!("vblank");
            irf.vblank = false;
            self.registers.interrupts_enabled = false;
            _call(self, 0x40);
        } else if irf.lcd_stat && ire.lcd_stat {
            println!("lcd_stat");
            irf.lcd_stat = false;
            self.registers.interrupts_enabled = false;
            _call(self, 0x48);
        } else if irf.timer && ire.timer {
            println!("timer");
            irf.timer = false;
            self.registers.interrupts_enabled = false;
            _call(self, 0x50);
        } else if irf.serial && ire.serial {
            println!("serial");
            irf.serial = false;
            self.registers.interrupts_enabled = false;
            _call(self, 0x58);
        } else if irf.joypad && ire.joypad {
            println!("joypad");
            irf.joypad = false;
            self.registers.interrupts_enabled = false;
            _call(self, 0x60);
        }
    }

    pub fn next_instruction(&mut self) -> Option<(Mode, Mode)> {
        if self.registers.interrupts_enabled {
            self.handle_interrupts();
        }

        let opcode = self.fetch_opcode();
        if opcode == 0x00FC {
            println!("DEBUG: {:?}", self.registers);
        }
        let result = self.execute(opcode);

        // println!("DEBUG: {:?}", result.name);
        // println!("DEBUG: {:?}", self.registers);

        self.mmu.lcd.advance_cycles(result.cycles)
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn halt(&mut self) {
        self.halted = true;
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

    pub fn execute(&mut self, opcode: u16) -> instructions::OpResult {
        match opcode {
            0x0000 => instructions::nop(self),
            0x0001 => instructions::ld_r16_n16(self, Registers16::BC),
            0x0002 => instructions::ld_ar16_r8(self, Registers16::BC, Registers8::A),
            0x0003 => instructions::inc_r16(self, Registers16::BC),
            0x0004 => instructions::inc_r8(self, Registers8::B),
            0x0005 => instructions::dec_r8(self, Registers8::B),
            0x0006 => instructions::ld_r8_n8(self, Registers8::B),
            0x0007 => instructions::rlca(self),
            0x0008 => instructions::ld_an16_r16(self, Registers16::SP),
            0x0009 => instructions::add_r16_r16(self, Registers16::HL, Registers16::BC),
            0x000A => instructions::ld_r8_ar16(self, Registers8::A, Registers16::BC),
            0x000B => instructions::dec_r16(self, Registers16::BC),
            0x000C => instructions::inc_r8(self, Registers8::C),
            0x000D => instructions::dec_r8(self, Registers8::C),
            0x000E => instructions::ld_r8_n8(self, Registers8::C),
            0x000F => instructions::rrca(self),

            0x0010 => instructions::stop(self),
            0x0011 => instructions::ld_r16_n16(self, Registers16::DE),
            0x0012 => instructions::ld_ar16_r8(self, Registers16::DE, Registers8::A),
            0x0013 => instructions::inc_r16(self, Registers16::DE),
            0x0014 => instructions::inc_r8(self, Registers8::D),
            0x0015 => instructions::dec_r8(self, Registers8::D),
            0x0016 => instructions::ld_r8_n8(self, Registers8::D),
            0x0017 => instructions::rla(self),
            0x0018 => instructions::jr_n8(self),
            0x0019 => instructions::add_r16_r16(self, Registers16::HL, Registers16::DE),
            0x001A => instructions::ld_r8_ar16(self, Registers8::A, Registers16::DE),
            0x001B => instructions::dec_r16(self, Registers16::DE),
            0x001C => instructions::inc_r8(self, Registers8::E),
            0x001D => instructions::dec_r8(self, Registers8::E),
            0x001E => instructions::ld_r8_n8(self, Registers8::E),
            0x001F => instructions::rra(self),

            0x0020 => instructions::jr_f_n8(self, JumpFlag::NZ),
            0x0021 => instructions::ld_r16_n16(self, Registers16::HL),
            0x0022 => instructions::ldi_ar16_r8(self, Registers16::HL, Registers8::A),
            0x0023 => instructions::inc_r16(self, Registers16::HL),
            0x0024 => instructions::inc_r8(self, Registers8::H),
            0x0025 => instructions::dec_r8(self, Registers8::H),
            0x0026 => instructions::ld_r8_n8(self, Registers8::H),
            0x0027 => instructions::daa(self),
            0x0028 => instructions::jr_f_n8(self, JumpFlag::Z),
            0x0029 => instructions::add_r16_r16(self, Registers16::HL, Registers16::HL),
            0x002A => instructions::ldi_r8_ar16(self, Registers8::A, Registers16::HL),
            0x002B => instructions::dec_r16(self, Registers16::HL),
            0x002C => instructions::inc_r8(self, Registers8::L),
            0x002D => instructions::dec_r8(self, Registers8::L),
            0x002E => instructions::ld_r8_n8(self, Registers8::L),
            0x002F => instructions::cpl(self),

            0x0030 => instructions::jr_f_n8(self, JumpFlag::NC),
            0x0031 => instructions::ld_r16_n16(self, Registers16::SP),
            0x0032 => instructions::ldd_ar16_r8(self, Registers16::HL, Registers8::A),
            0x0033 => instructions::inc_r16(self, Registers16::SP),
            0x0034 => instructions::inc_ar16(self, Registers16::HL),
            0x0035 => instructions::dec_ar16(self, Registers16::HL),
            0x0036 => instructions::ld_ar16_n8(self, Registers16::HL),
            0x0037 => instructions::scf(self),
            0x0038 => instructions::jr_f_n8(self, JumpFlag::C),
            0x0039 => instructions::add_r16_r16(self, Registers16::HL, Registers16::SP),
            0x003A => instructions::ldd_r8_ar16(self, Registers8::A, Registers16::HL),
            0x003B => instructions::dec_r16(self, Registers16::SP),
            0x003C => instructions::inc_r8(self, Registers8::A),
            0x003D => instructions::dec_r8(self, Registers8::A),
            0x003E => instructions::ld_r8_n8(self, Registers8::A),
            0x003F => instructions::ccf(self),

            0x0040 => instructions::ld_r8_r8(self, Registers8::B, Registers8::B),
            0x0041 => instructions::ld_r8_r8(self, Registers8::B, Registers8::C),
            0x0042 => instructions::ld_r8_r8(self, Registers8::B, Registers8::D),
            0x0043 => instructions::ld_r8_r8(self, Registers8::B, Registers8::E),
            0x0044 => instructions::ld_r8_r8(self, Registers8::B, Registers8::H),
            0x0045 => instructions::ld_r8_r8(self, Registers8::B, Registers8::L),
            0x0046 => instructions::ld_r8_ar16(self, Registers8::B, Registers16::HL),
            0x0047 => instructions::ld_r8_r8(self, Registers8::B, Registers8::A),

            0x0048 => instructions::ld_r8_r8(self, Registers8::C, Registers8::B),
            0x0049 => instructions::ld_r8_r8(self, Registers8::C, Registers8::C),
            0x004A => instructions::ld_r8_r8(self, Registers8::C, Registers8::D),
            0x004B => instructions::ld_r8_r8(self, Registers8::C, Registers8::E),
            0x004C => instructions::ld_r8_r8(self, Registers8::C, Registers8::H),
            0x004D => instructions::ld_r8_r8(self, Registers8::C, Registers8::L),
            0x004E => instructions::ld_r8_ar16(self, Registers8::C, Registers16::HL),
            0x004F => instructions::ld_r8_r8(self, Registers8::C, Registers8::A),

            0x0050 => instructions::ld_r8_r8(self, Registers8::D, Registers8::B),
            0x0051 => instructions::ld_r8_r8(self, Registers8::D, Registers8::C),
            0x0052 => instructions::ld_r8_r8(self, Registers8::D, Registers8::D),
            0x0053 => instructions::ld_r8_r8(self, Registers8::D, Registers8::E),
            0x0054 => instructions::ld_r8_r8(self, Registers8::D, Registers8::H),
            0x0055 => instructions::ld_r8_r8(self, Registers8::D, Registers8::L),
            0x0056 => instructions::ld_r8_ar16(self, Registers8::D, Registers16::HL),
            0x0057 => instructions::ld_r8_r8(self, Registers8::D, Registers8::A),

            0x0058 => instructions::ld_r8_r8(self, Registers8::E, Registers8::B),
            0x0059 => instructions::ld_r8_r8(self, Registers8::E, Registers8::C),
            0x005A => instructions::ld_r8_r8(self, Registers8::E, Registers8::D),
            0x005B => instructions::ld_r8_r8(self, Registers8::E, Registers8::E),
            0x005C => instructions::ld_r8_r8(self, Registers8::E, Registers8::H),
            0x005D => instructions::ld_r8_r8(self, Registers8::E, Registers8::L),
            0x005E => instructions::ld_r8_ar16(self, Registers8::E, Registers16::HL),
            0x005F => instructions::ld_r8_r8(self, Registers8::E, Registers8::A),

            0x0060 => instructions::ld_r8_r8(self, Registers8::H, Registers8::B),
            0x0061 => instructions::ld_r8_r8(self, Registers8::H, Registers8::C),
            0x0062 => instructions::ld_r8_r8(self, Registers8::H, Registers8::D),
            0x0063 => instructions::ld_r8_r8(self, Registers8::H, Registers8::E),
            0x0064 => instructions::ld_r8_r8(self, Registers8::H, Registers8::H),
            0x0065 => instructions::ld_r8_r8(self, Registers8::H, Registers8::L),
            0x0066 => instructions::ld_r8_ar16(self, Registers8::H, Registers16::HL),
            0x0067 => instructions::ld_r8_r8(self, Registers8::H, Registers8::A),

            0x0068 => instructions::ld_r8_r8(self, Registers8::L, Registers8::B),
            0x0069 => instructions::ld_r8_r8(self, Registers8::L, Registers8::C),
            0x006A => instructions::ld_r8_r8(self, Registers8::L, Registers8::D),
            0x006B => instructions::ld_r8_r8(self, Registers8::L, Registers8::E),
            0x006C => instructions::ld_r8_r8(self, Registers8::L, Registers8::H),
            0x006D => instructions::ld_r8_r8(self, Registers8::L, Registers8::L),
            0x006E => instructions::ld_r8_ar16(self, Registers8::L, Registers16::HL),
            0x006F => instructions::ld_r8_r8(self, Registers8::L, Registers8::A),

            0x0070 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::B),
            0x0071 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::C),
            0x0072 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::D),
            0x0073 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::E),
            0x0074 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::H),
            0x0075 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::L),
            0x0076 => instructions::halt(self),
            0x0077 => instructions::ld_ar16_r8(self, Registers16::HL, Registers8::A),

            0x0078 => instructions::ld_r8_r8(self, Registers8::A, Registers8::B),
            0x0079 => instructions::ld_r8_r8(self, Registers8::A, Registers8::C),
            0x007A => instructions::ld_r8_r8(self, Registers8::A, Registers8::D),
            0x007B => instructions::ld_r8_r8(self, Registers8::A, Registers8::E),
            0x007C => instructions::ld_r8_r8(self, Registers8::A, Registers8::H),
            0x007D => instructions::ld_r8_r8(self, Registers8::A, Registers8::L),
            0x007E => instructions::ld_r8_ar16(self, Registers8::A, Registers16::HL),
            0x007F => instructions::ld_r8_r8(self, Registers8::A, Registers8::A),

            0x0080 => instructions::add_r8_r8(self, Registers8::A, Registers8::B), 
            0x0081 => instructions::add_r8_r8(self, Registers8::A, Registers8::C), 
            0x0082 => instructions::add_r8_r8(self, Registers8::A, Registers8::D), 
            0x0083 => instructions::add_r8_r8(self, Registers8::A, Registers8::E), 
            0x0084 => instructions::add_r8_r8(self, Registers8::A, Registers8::H), 
            0x0085 => instructions::add_r8_r8(self, Registers8::A, Registers8::L), 
            0x0086 => instructions::add_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x0087 => instructions::add_r8_r8(self, Registers8::A, Registers8::A), 

            0x0088 => instructions::adc_r8_r8(self, Registers8::A, Registers8::B), 
            0x0089 => instructions::adc_r8_r8(self, Registers8::A, Registers8::C), 
            0x008A => instructions::adc_r8_r8(self, Registers8::A, Registers8::D), 
            0x008B => instructions::adc_r8_r8(self, Registers8::A, Registers8::E), 
            0x008C => instructions::adc_r8_r8(self, Registers8::A, Registers8::H), 
            0x008D => instructions::adc_r8_r8(self, Registers8::A, Registers8::L), 
            0x008E => instructions::adc_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x008F => instructions::adc_r8_r8(self, Registers8::A, Registers8::A), 

            0x0090 => instructions::sub_r8_r8(self, Registers8::A, Registers8::B), 
            0x0091 => instructions::sub_r8_r8(self, Registers8::A, Registers8::C), 
            0x0092 => instructions::sub_r8_r8(self, Registers8::A, Registers8::D), 
            0x0093 => instructions::sub_r8_r8(self, Registers8::A, Registers8::E), 
            0x0094 => instructions::sub_r8_r8(self, Registers8::A, Registers8::H), 
            0x0095 => instructions::sub_r8_r8(self, Registers8::A, Registers8::L), 
            0x0096 => instructions::sub_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x0097 => instructions::sub_r8_r8(self, Registers8::A, Registers8::A), 

            0x0098 => instructions::sbc_r8_r8(self, Registers8::A, Registers8::B), 
            0x0099 => instructions::sbc_r8_r8(self, Registers8::A, Registers8::C), 
            0x009A => instructions::sbc_r8_r8(self, Registers8::A, Registers8::D), 
            0x009B => instructions::sbc_r8_r8(self, Registers8::A, Registers8::E), 
            0x009C => instructions::sbc_r8_r8(self, Registers8::A, Registers8::H), 
            0x009D => instructions::sbc_r8_r8(self, Registers8::A, Registers8::L), 
            0x009E => instructions::sbc_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x009F => instructions::sbc_r8_r8(self, Registers8::A, Registers8::A), 

            0x00A0 => instructions::and_r8_r8(self, Registers8::A, Registers8::B), 
            0x00A1 => instructions::and_r8_r8(self, Registers8::A, Registers8::C), 
            0x00A2 => instructions::and_r8_r8(self, Registers8::A, Registers8::D), 
            0x00A3 => instructions::and_r8_r8(self, Registers8::A, Registers8::E), 
            0x00A4 => instructions::and_r8_r8(self, Registers8::A, Registers8::H), 
            0x00A5 => instructions::and_r8_r8(self, Registers8::A, Registers8::L), 
            0x00A6 => instructions::and_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x00A7 => instructions::and_r8_r8(self, Registers8::A, Registers8::A), 

            0x00A8 => instructions::xor_r8_r8(self, Registers8::A, Registers8::B), 
            0x00A9 => instructions::xor_r8_r8(self, Registers8::A, Registers8::C), 
            0x00AA => instructions::xor_r8_r8(self, Registers8::A, Registers8::D), 
            0x00AB => instructions::xor_r8_r8(self, Registers8::A, Registers8::E), 
            0x00AC => instructions::xor_r8_r8(self, Registers8::A, Registers8::H), 
            0x00AD => instructions::xor_r8_r8(self, Registers8::A, Registers8::L), 
            0x00AE => instructions::xor_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x00AF => instructions::xor_r8_r8(self, Registers8::A, Registers8::A), 

            0x00B0 => instructions::or_r8_r8(self, Registers8::A, Registers8::B), 
            0x00B1 => instructions::or_r8_r8(self, Registers8::A, Registers8::C), 
            0x00B2 => instructions::or_r8_r8(self, Registers8::A, Registers8::D), 
            0x00B3 => instructions::or_r8_r8(self, Registers8::A, Registers8::E), 
            0x00B4 => instructions::or_r8_r8(self, Registers8::A, Registers8::H), 
            0x00B5 => instructions::or_r8_r8(self, Registers8::A, Registers8::L), 
            0x00B6 => instructions::or_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x00B7 => instructions::or_r8_r8(self, Registers8::A, Registers8::A), 

            0x00B8 => instructions::cp_r8_r8(self, Registers8::A, Registers8::B), 
            0x00B9 => instructions::cp_r8_r8(self, Registers8::A, Registers8::C), 
            0x00BA => instructions::cp_r8_r8(self, Registers8::A, Registers8::D), 
            0x00BB => instructions::cp_r8_r8(self, Registers8::A, Registers8::E), 
            0x00BC => instructions::cp_r8_r8(self, Registers8::A, Registers8::H), 
            0x00BD => instructions::cp_r8_r8(self, Registers8::A, Registers8::L), 
            0x00BE => instructions::cp_r8_ar16(self, Registers8::A, Registers16::HL), 
            0x00BF => instructions::cp_r8_r8(self, Registers8::A, Registers8::A), 

            0x00C0 => instructions::ret_f(self, JumpFlag::NZ),
            0x00C1 => instructions::pop_r16(self, Registers16::BC),
            0x00C2 => instructions::jp_f_n16(self, JumpFlag::NZ),
            0x00C3 => instructions::jp_n16(self),
            0x00C4 => instructions::call_f_n16(self, JumpFlag::NZ),
            0x00C5 => instructions::push_r16(self, Registers16::BC),
            0x00C6 => instructions::add_r8_n8(self, Registers8::A),
            0x00C7 => instructions::rst_f(self, RstFlag::H00),

            0x00C8 => instructions::ret_f(self, JumpFlag::Z),
            0x00C9 => instructions::ret(self),
            0x00CA => instructions::jp_f_n16(self, JumpFlag::Z),
            0x00CB => instructions::illegal_opcode("CB"),
            0x00CC => instructions::call_f_n16(self, JumpFlag::Z),
            0x00CD => instructions::call_n16(self),
            0x00CE => instructions::adc_r8_n8(self, Registers8::A),
            0x00CF => instructions::rst_f(self, RstFlag::H08),

            0x00D0 => instructions::ret_f(self, JumpFlag::NC),
            0x00D1 => instructions::pop_r16(self, Registers16::DE),
            0x00D2 => instructions::jp_f_n16(self, JumpFlag::NC),
            0x00D3 => instructions::illegal_opcode("D3"),
            0x00D4 => instructions::call_f_n16(self, JumpFlag::NC),
            0x00D5 => instructions::push_r16(self, Registers16::DE),
            0x00D6 => instructions::sub_r8_n8(self, Registers8::A),
            0x00D7 => instructions::rst_f(self, RstFlag::H10),

            0x00D8 => instructions::ret_f(self, JumpFlag::C),
            0x00D9 => instructions::reti(self),
            0x00DA => instructions::jp_f_n16(self, JumpFlag::C),
            0x00DB => instructions::illegal_opcode("DB"),
            0x00DC => instructions::call_f_n16(self, JumpFlag::C),
            0x00DD => instructions::illegal_opcode("DD"),
            0x00DE => instructions::sbc_r8_n8(self, Registers8::A),
            0x00DF => instructions::rst_f(self, RstFlag::H18),

            0x00E0 => instructions::ldh_an8_r8(self, Registers8::A),
            0x00E1 => instructions::pop_r16(self, Registers16::HL),
            0x00E2 => instructions::ldc_ar8_r8(self, Registers8::C, Registers8::A),
            0x00E3 => instructions::illegal_opcode("E3"),
            0x00E4 => instructions::illegal_opcode("E4"),
            0x00E5 => instructions::push_r16(self, Registers16::HL),
            0x00E6 => instructions::and_r8_n8(self, Registers8::A),
            0x00E7 => instructions::rst_f(self, RstFlag::H20),

            0x00E8 => instructions::add_r16_n8(self, Registers16::SP),
            0x00E9 => instructions::jp_r16(self, Registers16::HL),
            0x00EA => instructions::ld_an16_r8(self, Registers8::A),
            0x00EB => instructions::illegal_opcode("EB"),
            0x00EC => instructions::illegal_opcode("EC"),
            0x00ED => instructions::illegal_opcode("ED"),
            0x00EE => instructions::xor_r8_n8(self, Registers8::A),
            0x00EF => instructions::rst_f(self, RstFlag::H28),

            0x00F0 => instructions::ldh_r8_an8(self, Registers8::A),
            0x00F1 => instructions::pop_r16(self, Registers16::AF),
            0x00F2 => instructions::ldc_r8_ar8(self, Registers8::A, Registers8::C),
            0x00F3 => instructions::di(self),
            0x00F4 => instructions::illegal_opcode("F4"),
            0x00F5 => instructions::push_r16(self, Registers16::AF),
            0x00F6 => instructions::or_r8_n8(self, Registers8::A),
            0x00F7 => instructions::rst_f(self, RstFlag::H30),

            0x00F8 => instructions::ld_r16_spn8(self, Registers16::HL),
            0x00F9 => instructions::ld_r16_r16(self, Registers16::SP, Registers16::HL),
            0x00FA => instructions::ld_r8_an16(self, Registers8::A),
            0x00FB => instructions::ei(self),
            0x00FC => instructions::illegal_opcode("FC"),
            0x00FD => instructions::illegal_opcode("FD"),
            0x00FE => instructions::cp_r8_n8(self, Registers8::A),
            0x00FF => instructions::rst_f(self, RstFlag::H38),

            // Prefix instructions::CB

            0xCB00 => instructions::rlc_r8(self, Registers8::B),
            0xCB01 => instructions::rlc_r8(self, Registers8::C),
            0xCB02 => instructions::rlc_r8(self, Registers8::D),
            0xCB03 => instructions::rlc_r8(self, Registers8::E),
            0xCB04 => instructions::rlc_r8(self, Registers8::H),
            0xCB05 => instructions::rlc_r8(self, Registers8::L),
            0xCB06 => instructions::rlc_ar16(self, Registers16::HL),
            0xCB07 => instructions::rlc_r8(self, Registers8::A),

            0xCB08 => instructions::rrc_r8(self, Registers8::B),
            0xCB09 => instructions::rrc_r8(self, Registers8::C),
            0xCB0A => instructions::rrc_r8(self, Registers8::D),
            0xCB0B => instructions::rrc_r8(self, Registers8::E),
            0xCB0C => instructions::rrc_r8(self, Registers8::H),
            0xCB0D => instructions::rrc_r8(self, Registers8::L),
            0xCB0E => instructions::rrc_ar16(self, Registers16::HL),
            0xCB0F => instructions::rrc_r8(self, Registers8::A),

            0xCB10 => instructions::rl_r8(self, Registers8::B),
            0xCB11 => instructions::rl_r8(self, Registers8::C),
            0xCB12 => instructions::rl_r8(self, Registers8::D),
            0xCB13 => instructions::rl_r8(self, Registers8::E),
            0xCB14 => instructions::rl_r8(self, Registers8::H),
            0xCB15 => instructions::rl_r8(self, Registers8::L),
            0xCB16 => instructions::rl_ar16(self, Registers16::HL),
            0xCB17 => instructions::rl_r8(self, Registers8::A),

            0xCB18 => instructions::rr_r8(self, Registers8::B),
            0xCB19 => instructions::rr_r8(self, Registers8::C),
            0xCB1A => instructions::rr_r8(self, Registers8::D),
            0xCB1B => instructions::rr_r8(self, Registers8::E),
            0xCB1C => instructions::rr_r8(self, Registers8::H),
            0xCB1D => instructions::rr_r8(self, Registers8::L),
            0xCB1E => instructions::rr_ar16(self, Registers16::HL),
            0xCB1F => instructions::rr_r8(self, Registers8::A),

            0xCB20 => instructions::sla_r8(self, Registers8::B),
            0xCB21 => instructions::sla_r8(self, Registers8::C),
            0xCB22 => instructions::sla_r8(self, Registers8::D),
            0xCB23 => instructions::sla_r8(self, Registers8::E),
            0xCB24 => instructions::sla_r8(self, Registers8::H),
            0xCB25 => instructions::sla_r8(self, Registers8::L),
            0xCB26 => instructions::sla_ar16(self, Registers16::HL),
            0xCB27 => instructions::sla_r8(self, Registers8::A),

            0xCB28 => instructions::sra_r8(self, Registers8::B),
            0xCB29 => instructions::sra_r8(self, Registers8::C),
            0xCB2A => instructions::sra_r8(self, Registers8::D),
            0xCB2B => instructions::sra_r8(self, Registers8::E),
            0xCB2C => instructions::sra_r8(self, Registers8::H),
            0xCB2D => instructions::sra_r8(self, Registers8::L),
            0xCB2E => instructions::sra_ar16(self, Registers16::HL),
            0xCB2F => instructions::sra_r8(self, Registers8::A),

            0xCB30 => instructions::swap_r8(self, Registers8::B),
            0xCB31 => instructions::swap_r8(self, Registers8::C),
            0xCB32 => instructions::swap_r8(self, Registers8::D),
            0xCB33 => instructions::swap_r8(self, Registers8::E),
            0xCB34 => instructions::swap_r8(self, Registers8::H),
            0xCB35 => instructions::swap_r8(self, Registers8::L),
            0xCB36 => instructions::swap_ar16(self, Registers16::HL),
            0xCB37 => instructions::swap_r8(self, Registers8::A),

            0xCB38 => instructions::srl_r8(self, Registers8::B),
            0xCB39 => instructions::srl_r8(self, Registers8::C),
            0xCB3A => instructions::srl_r8(self, Registers8::D),
            0xCB3B => instructions::srl_r8(self, Registers8::E),
            0xCB3C => instructions::srl_r8(self, Registers8::H),
            0xCB3D => instructions::srl_r8(self, Registers8::L),
            0xCB3E => instructions::srl_ar16(self, Registers16::HL),
            0xCB3F => instructions::srl_r8(self, Registers8::A),

            0xCB40 => instructions::bit_r8(self, 0, Registers8::B),
            0xCB41 => instructions::bit_r8(self, 0, Registers8::C),
            0xCB42 => instructions::bit_r8(self, 0, Registers8::D),
            0xCB43 => instructions::bit_r8(self, 0, Registers8::E),
            0xCB44 => instructions::bit_r8(self, 0, Registers8::H),
            0xCB45 => instructions::bit_r8(self, 0, Registers8::L),
            0xCB46 => instructions::bit_ar16(self, 0, Registers16::HL),
            0xCB47 => instructions::bit_r8(self, 0, Registers8::A),

            0xCB48 => instructions::bit_r8(self, 1, Registers8::B),
            0xCB49 => instructions::bit_r8(self, 1, Registers8::C),
            0xCB4A => instructions::bit_r8(self, 1, Registers8::D),
            0xCB4B => instructions::bit_r8(self, 1, Registers8::E),
            0xCB4C => instructions::bit_r8(self, 1, Registers8::H),
            0xCB4D => instructions::bit_r8(self, 1, Registers8::L),
            0xCB4E => instructions::bit_ar16(self, 1, Registers16::HL),
            0xCB4F => instructions::bit_r8(self, 1, Registers8::A),

            0xCB50 => instructions::bit_r8(self, 2, Registers8::B),
            0xCB51 => instructions::bit_r8(self, 2, Registers8::C),
            0xCB52 => instructions::bit_r8(self, 2, Registers8::D),
            0xCB53 => instructions::bit_r8(self, 2, Registers8::E),
            0xCB54 => instructions::bit_r8(self, 2, Registers8::H),
            0xCB55 => instructions::bit_r8(self, 2, Registers8::L),
            0xCB56 => instructions::bit_ar16(self, 2, Registers16::HL),
            0xCB57 => instructions::bit_r8(self, 2, Registers8::A),

            0xCB58 => instructions::bit_r8(self, 3, Registers8::B),
            0xCB59 => instructions::bit_r8(self, 3, Registers8::C),
            0xCB5A => instructions::bit_r8(self, 3, Registers8::D),
            0xCB5B => instructions::bit_r8(self, 3, Registers8::E),
            0xCB5C => instructions::bit_r8(self, 3, Registers8::H),
            0xCB5D => instructions::bit_r8(self, 3, Registers8::L),
            0xCB5E => instructions::bit_ar16(self, 3, Registers16::HL),
            0xCB5F => instructions::bit_r8(self, 3, Registers8::A),

            0xCB60 => instructions::bit_r8(self, 4, Registers8::B),
            0xCB61 => instructions::bit_r8(self, 4, Registers8::C),
            0xCB62 => instructions::bit_r8(self, 4, Registers8::D),
            0xCB63 => instructions::bit_r8(self, 4, Registers8::E),
            0xCB64 => instructions::bit_r8(self, 4, Registers8::H),
            0xCB65 => instructions::bit_r8(self, 4, Registers8::L),
            0xCB66 => instructions::bit_ar16(self, 4, Registers16::HL),
            0xCB67 => instructions::bit_r8(self, 4, Registers8::A),

            0xCB68 => instructions::bit_r8(self, 5, Registers8::B),
            0xCB69 => instructions::bit_r8(self, 5, Registers8::C),
            0xCB6A => instructions::bit_r8(self, 5, Registers8::D),
            0xCB6B => instructions::bit_r8(self, 5, Registers8::E),
            0xCB6C => instructions::bit_r8(self, 5, Registers8::H),
            0xCB6D => instructions::bit_r8(self, 5, Registers8::L),
            0xCB6E => instructions::bit_ar16(self, 5, Registers16::HL),
            0xCB6F => instructions::bit_r8(self, 5, Registers8::A),

            0xCB70 => instructions::bit_r8(self, 6, Registers8::B),
            0xCB71 => instructions::bit_r8(self, 6, Registers8::C),
            0xCB72 => instructions::bit_r8(self, 6, Registers8::D),
            0xCB73 => instructions::bit_r8(self, 6, Registers8::E),
            0xCB74 => instructions::bit_r8(self, 6, Registers8::H),
            0xCB75 => instructions::bit_r8(self, 6, Registers8::L),
            0xCB76 => instructions::bit_ar16(self, 6, Registers16::HL),
            0xCB77 => instructions::bit_r8(self, 6, Registers8::A),

            0xCB78 => instructions::bit_r8(self, 7, Registers8::B),
            0xCB79 => instructions::bit_r8(self, 7, Registers8::C),
            0xCB7A => instructions::bit_r8(self, 7, Registers8::D),
            0xCB7B => instructions::bit_r8(self, 7, Registers8::E),
            0xCB7C => instructions::bit_r8(self, 7, Registers8::H),
            0xCB7D => instructions::bit_r8(self, 7, Registers8::L),
            0xCB7E => instructions::bit_ar16(self, 7, Registers16::HL),
            0xCB7F => instructions::bit_r8(self, 7, Registers8::A),

            0xCB80 => instructions::res_r8(self, 0, Registers8::B),
            0xCB81 => instructions::res_r8(self, 0, Registers8::C),
            0xCB82 => instructions::res_r8(self, 0, Registers8::D),
            0xCB83 => instructions::res_r8(self, 0, Registers8::E),
            0xCB84 => instructions::res_r8(self, 0, Registers8::H),
            0xCB85 => instructions::res_r8(self, 0, Registers8::L),
            0xCB86 => instructions::res_ar16(self, 0, Registers16::HL),
            0xCB87 => instructions::res_r8(self, 0, Registers8::A),

            0xCB88 => instructions::res_r8(self, 1, Registers8::B),
            0xCB89 => instructions::res_r8(self, 1, Registers8::C),
            0xCB8A => instructions::res_r8(self, 1, Registers8::D),
            0xCB8B => instructions::res_r8(self, 1, Registers8::E),
            0xCB8C => instructions::res_r8(self, 1, Registers8::H),
            0xCB8D => instructions::res_r8(self, 1, Registers8::L),
            0xCB8E => instructions::res_ar16(self, 1, Registers16::HL),
            0xCB8F => instructions::res_r8(self, 1, Registers8::A),

            0xCB90 => instructions::res_r8(self, 2, Registers8::B),
            0xCB91 => instructions::res_r8(self, 2, Registers8::C),
            0xCB92 => instructions::res_r8(self, 2, Registers8::D),
            0xCB93 => instructions::res_r8(self, 2, Registers8::E),
            0xCB94 => instructions::res_r8(self, 2, Registers8::H),
            0xCB95 => instructions::res_r8(self, 2, Registers8::L),
            0xCB96 => instructions::res_ar16(self, 2, Registers16::HL),
            0xCB97 => instructions::res_r8(self, 2, Registers8::A),

            0xCB98 => instructions::res_r8(self, 3, Registers8::B),
            0xCB99 => instructions::res_r8(self, 3, Registers8::C),
            0xCB9A => instructions::res_r8(self, 3, Registers8::D),
            0xCB9B => instructions::res_r8(self, 3, Registers8::E),
            0xCB9C => instructions::res_r8(self, 3, Registers8::H),
            0xCB9D => instructions::res_r8(self, 3, Registers8::L),
            0xCB9E => instructions::res_ar16(self, 3, Registers16::HL),
            0xCB9F => instructions::res_r8(self, 3, Registers8::A),

            0xCBA0 => instructions::res_r8(self, 4, Registers8::B),
            0xCBA1 => instructions::res_r8(self, 4, Registers8::C),
            0xCBA2 => instructions::res_r8(self, 4, Registers8::D),
            0xCBA3 => instructions::res_r8(self, 4, Registers8::E),
            0xCBA4 => instructions::res_r8(self, 4, Registers8::H),
            0xCBA5 => instructions::res_r8(self, 4, Registers8::L),
            0xCBA6 => instructions::res_ar16(self, 4, Registers16::HL),
            0xCBA7 => instructions::res_r8(self, 4, Registers8::A),

            0xCBA8 => instructions::res_r8(self, 5, Registers8::B),
            0xCBA9 => instructions::res_r8(self, 5, Registers8::C),
            0xCBAA => instructions::res_r8(self, 5, Registers8::D),
            0xCBAB => instructions::res_r8(self, 5, Registers8::E),
            0xCBAC => instructions::res_r8(self, 5, Registers8::H),
            0xCBAD => instructions::res_r8(self, 5, Registers8::L),
            0xCBAE => instructions::res_ar16(self, 5, Registers16::HL),
            0xCBAF => instructions::res_r8(self, 5, Registers8::A),

            0xCBB0 => instructions::res_r8(self, 6, Registers8::B),
            0xCBB1 => instructions::res_r8(self, 6, Registers8::C),
            0xCBB2 => instructions::res_r8(self, 6, Registers8::D),
            0xCBB3 => instructions::res_r8(self, 6, Registers8::E),
            0xCBB4 => instructions::res_r8(self, 6, Registers8::H),
            0xCBB5 => instructions::res_r8(self, 6, Registers8::L),
            0xCBB6 => instructions::res_ar16(self, 6, Registers16::HL),
            0xCBB7 => instructions::res_r8(self, 6, Registers8::A),

            0xCBB8 => instructions::res_r8(self, 7, Registers8::B),
            0xCBB9 => instructions::res_r8(self, 7, Registers8::C),
            0xCBBA => instructions::res_r8(self, 7, Registers8::D),
            0xCBBB => instructions::res_r8(self, 7, Registers8::E),
            0xCBBC => instructions::res_r8(self, 7, Registers8::H),
            0xCBBD => instructions::res_r8(self, 7, Registers8::L),
            0xCBBE => instructions::res_ar16(self, 7, Registers16::HL),
            0xCBBF => instructions::res_r8(self, 7, Registers8::A),

            0xCBC0 => instructions::set_r8(self, 0, Registers8::B),
            0xCBC1 => instructions::set_r8(self, 0, Registers8::C),
            0xCBC2 => instructions::set_r8(self, 0, Registers8::D),
            0xCBC3 => instructions::set_r8(self, 0, Registers8::E),
            0xCBC4 => instructions::set_r8(self, 0, Registers8::H),
            0xCBC5 => instructions::set_r8(self, 0, Registers8::L),
            0xCBC6 => instructions::set_ar16(self, 0, Registers16::HL),
            0xCBC7 => instructions::set_r8(self, 0, Registers8::A),

            0xCBC8 => instructions::set_r8(self, 1, Registers8::B),
            0xCBC9 => instructions::set_r8(self, 1, Registers8::C),
            0xCBCA => instructions::set_r8(self, 1, Registers8::D),
            0xCBCB => instructions::set_r8(self, 1, Registers8::E),
            0xCBCC => instructions::set_r8(self, 1, Registers8::H),
            0xCBCD => instructions::set_r8(self, 1, Registers8::L),
            0xCBCE => instructions::set_ar16(self, 1, Registers16::HL),
            0xCBCF => instructions::set_r8(self, 1, Registers8::A),

            0xCBD0 => instructions::set_r8(self, 2, Registers8::B),
            0xCBD1 => instructions::set_r8(self, 2, Registers8::C),
            0xCBD2 => instructions::set_r8(self, 2, Registers8::D),
            0xCBD3 => instructions::set_r8(self, 2, Registers8::E),
            0xCBD4 => instructions::set_r8(self, 2, Registers8::H),
            0xCBD5 => instructions::set_r8(self, 2, Registers8::L),
            0xCBD6 => instructions::set_ar16(self, 2, Registers16::HL),
            0xCBD7 => instructions::set_r8(self, 2, Registers8::A),

            0xCBD8 => instructions::set_r8(self, 3, Registers8::B),
            0xCBD9 => instructions::set_r8(self, 3, Registers8::C),
            0xCBDA => instructions::set_r8(self, 3, Registers8::D),
            0xCBDB => instructions::set_r8(self, 3, Registers8::E),
            0xCBDC => instructions::set_r8(self, 3, Registers8::H),
            0xCBDD => instructions::set_r8(self, 3, Registers8::L),
            0xCBDE => instructions::set_ar16(self, 3, Registers16::HL),
            0xCBDF => instructions::set_r8(self, 3, Registers8::A),

            0xCBE0 => instructions::set_r8(self, 4, Registers8::B),
            0xCBE1 => instructions::set_r8(self, 4, Registers8::C),
            0xCBE2 => instructions::set_r8(self, 4, Registers8::D),
            0xCBE3 => instructions::set_r8(self, 4, Registers8::E),
            0xCBE4 => instructions::set_r8(self, 4, Registers8::H),
            0xCBE5 => instructions::set_r8(self, 4, Registers8::L),
            0xCBE6 => instructions::set_ar16(self, 4, Registers16::HL),
            0xCBE7 => instructions::set_r8(self, 4, Registers8::A),

            0xCBE8 => instructions::set_r8(self, 5, Registers8::B),
            0xCBE9 => instructions::set_r8(self, 5, Registers8::C),
            0xCBEA => instructions::set_r8(self, 5, Registers8::D),
            0xCBEB => instructions::set_r8(self, 5, Registers8::E),
            0xCBEC => instructions::set_r8(self, 5, Registers8::H),
            0xCBED => instructions::set_r8(self, 5, Registers8::L),
            0xCBEE => instructions::set_ar16(self, 5, Registers16::HL),
            0xCBEF => instructions::set_r8(self, 5, Registers8::A),

            0xCBF0 => instructions::set_r8(self, 6, Registers8::B),
            0xCBF1 => instructions::set_r8(self, 6, Registers8::C),
            0xCBF2 => instructions::set_r8(self, 6, Registers8::D),
            0xCBF3 => instructions::set_r8(self, 6, Registers8::E),
            0xCBF4 => instructions::set_r8(self, 6, Registers8::H),
            0xCBF5 => instructions::set_r8(self, 6, Registers8::L),
            0xCBF6 => instructions::set_ar16(self, 6, Registers16::HL),
            0xCBF7 => instructions::set_r8(self, 6, Registers8::A),

            0xCBF8 => instructions::set_r8(self, 7, Registers8::B),
            0xCBF9 => instructions::set_r8(self, 7, Registers8::C),
            0xCBFA => instructions::set_r8(self, 7, Registers8::D),
            0xCBFB => instructions::set_r8(self, 7, Registers8::E),
            0xCBFC => instructions::set_r8(self, 7, Registers8::H),
            0xCBFD => instructions::set_r8(self, 7, Registers8::L),
            0xCBFE => instructions::set_ar16(self, 7, Registers16::HL),
            0xCBFF => instructions::set_r8(self, 7, Registers8::A),

            _ => panic!("not implemented"),
        }
    }
}

