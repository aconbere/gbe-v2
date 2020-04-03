use crate::register::{Registers, Registers16, Registers8};
use crate::mmu::MMU;
use crate::bytes;
use crate::framebuffer::Framebuffer;
use crate::palette;

mod instructions;

use instructions::{JumpFlag, RstFlag};

const FRAME_CYCLES:u32 = 70244;

#[derive(PartialEq)]
pub enum Mode {
    // OAM Read mode
    OAM,

    // VRAM Read mode
    // End of VRAM is a completed scanline
    VRAM,

    // End of a scanline until the beginning of a new scanline
    // At the end of the last hblank we'll render our full frame
    HBlank,

    // End of a frame, vblank lasts ~10 lines
    VBlank,
}


pub struct CPU {
    mmu: MMU,
    registers: Registers,

    cycles: u32,
    mode_clock: u32,
    mode: Mode,
    lines: u32,
    frame_count: u32,

    pub framebuffer: Framebuffer,
    pub stopped: bool,
    pub halted: bool,
    interupts_enabled: bool,
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        CPU {
            mmu: mmu,
            registers: Registers::new(),

            cycles: 0,
            mode_clock: 0,
            mode: Mode::OAM,
            lines: 0,
            frame_count: 0,

            framebuffer: Framebuffer::new(),
            stopped: false,
            halted: false,
            interupts_enabled: false,
        }
    }

    pub fn next_frame(&mut self) {
        loop {
            if self.next_instruction() || self.stopped || self.halted {
                break;
            }
        }

        self.framebuffer.reset();

        self.framebuffer.set(1000, palette::Shade::Black);
        self.framebuffer.set(1001, palette::Shade::LightGrey);
        self.framebuffer.set(1002, palette::Shade::DarkGrey);
    }

    pub fn fetch_opcode(&mut self) -> u16{
        let opcode = self.advance_pc() as u16;

        /* the gameboy has two opcode spaces, the second space
         * is indicated by starting with the CB opcode. We store
         * the Prefixed opcodes with the byte prefix 01
         */
        if opcode == 0x00CB {
            self.fetch_opcode() | 0x0100
        } else {
            opcode
        }
    }

    pub fn next_instruction(&mut self) -> bool {
        let opcode = self.fetch_opcode();
        let result = self.execute(opcode);

        println!("DEBUG: {:?}", result.name);
        println!("DEBUG: {:?}", self.registers);

        self.advance_cycles(result.cycles)
    }


    pub fn enable_interrupts(&mut self) {
        self.interupts_enabled = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interupts_enabled = false;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn advance_cycles(&mut self, n: u8) -> bool {
        self.cycles = self.cycles.wrapping_add(n as u32);

        match self.mode {
            Mode::OAM => {
                if self.mode_clock >= 80 {
                    self.mode = Mode::VRAM;
                }
            }
            Mode::VRAM => {
                if self.mode_clock >= 252 {
                    self.render_line();
                    self.mode = Mode::HBlank;
                }
            }
            Mode::HBlank => {
                if self.mode_clock >= 456 {
                    self.mode_clock -= 456;

                    self.lines += 1;

                    if self.lines == 144 {
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                }
            }
            Mode::VBlank => {
                if self.mode_clock >= 456 {
                    self.mode_clock -= 456;
                    self.lines += 1;
                }

                if self.lines == 153 {
                    self.lines = 0;
                    self.mode = Mode::OAM;
                }
            }
        }

        // If is a new frame (clock check)
        if self.cycles >= FRAME_CYCLES {
            // if we crossed 70244 we want to loop back around
            self.frame_count += 1;
            self.cycles -= FRAME_CYCLES;
            true
        } else {
            false
        }
    }

    fn render_line(&mut self) {
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.registers.get16(Registers16::PC);
        self.registers.inc_pc();
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
            0x00E9 => instructions::jp_ar16(self, Registers16::HL),
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

            0x00F8 => instructions::ld_r16_spn8(self, Registers16::SP),
            0x00F9 => instructions::ld_r16_r16(self, Registers16::SP, Registers16::HL),
            0x00FA => instructions::ld_r8_an16(self, Registers8::A),
            0x00FB => instructions::ei(self),
            0x00FC => instructions::illegal_opcode("FC"),
            0x00FD => instructions::illegal_opcode("FD"),
            0x00FE => instructions::cp_r8_n8(self, Registers8::A),
            0x00FF => instructions::rst_f(self, RstFlag::H38),

            // Prefix instructions::CB

            0x0100 => instructions::rlc_r8(self, Registers8::B),
            0x0101 => instructions::rlc_r8(self, Registers8::C),
            0x0102 => instructions::rlc_r8(self, Registers8::D),
            0x0103 => instructions::rlc_r8(self, Registers8::E),
            0x0104 => instructions::rlc_r8(self, Registers8::H),
            0x0105 => instructions::rlc_r8(self, Registers8::L),
            0x0106 => instructions::rlc_ar16(self, Registers16::HL),
            0x0107 => instructions::rlc_r8(self, Registers8::A),

            0x0108 => instructions::rrc_r8(self, Registers8::B),
            0x0109 => instructions::rrc_r8(self, Registers8::C),
            0x010A => instructions::rrc_r8(self, Registers8::D),
            0x010B => instructions::rrc_r8(self, Registers8::E),
            0x010C => instructions::rrc_r8(self, Registers8::H),
            0x010D => instructions::rrc_r8(self, Registers8::L),
            0x010E => instructions::rrc_ar16(self, Registers16::HL),
            0x010F => instructions::rrc_r8(self, Registers8::A),

            0x0110 => instructions::rl_r8(self, Registers8::B),
            0x0111 => instructions::rl_r8(self, Registers8::C),
            0x0112 => instructions::rl_r8(self, Registers8::D),
            0x0113 => instructions::rl_r8(self, Registers8::E),
            0x0114 => instructions::rl_r8(self, Registers8::H),
            0x0115 => instructions::rl_r8(self, Registers8::L),
            0x0116 => instructions::rl_ar16(self, Registers16::HL),
            0x0117 => instructions::rl_r8(self, Registers8::A),

            0x0118 => instructions::rr_r8(self, Registers8::B),
            0x0119 => instructions::rr_r8(self, Registers8::C),
            0x011A => instructions::rr_r8(self, Registers8::D),
            0x011B => instructions::rr_r8(self, Registers8::E),
            0x011C => instructions::rr_r8(self, Registers8::H),
            0x011D => instructions::rr_r8(self, Registers8::L),
            0x011E => instructions::rr_ar16(self, Registers16::HL),
            0x011F => instructions::rr_r8(self, Registers8::A),

            0x0120 => instructions::sla_r8(self, Registers8::B),
            0x0121 => instructions::sla_r8(self, Registers8::C),
            0x0122 => instructions::sla_r8(self, Registers8::D),
            0x0123 => instructions::sla_r8(self, Registers8::E),
            0x0124 => instructions::sla_r8(self, Registers8::H),
            0x0125 => instructions::sla_r8(self, Registers8::L),
            0x0126 => instructions::sla_ar16(self, Registers16::HL),
            0x0127 => instructions::sla_r8(self, Registers8::A),

            0x0128 => instructions::sra_r8(self, Registers8::B),
            0x0129 => instructions::sra_r8(self, Registers8::C),
            0x012A => instructions::sra_r8(self, Registers8::D),
            0x012B => instructions::sra_r8(self, Registers8::E),
            0x012C => instructions::sra_r8(self, Registers8::H),
            0x012D => instructions::sra_r8(self, Registers8::L),
            0x012E => instructions::sra_ar16(self, Registers16::HL),
            0x012F => instructions::sra_r8(self, Registers8::A),

            0x0130 => instructions::swap_r8(self, Registers8::B),
            0x0131 => instructions::swap_r8(self, Registers8::C),
            0x0132 => instructions::swap_r8(self, Registers8::D),
            0x0133 => instructions::swap_r8(self, Registers8::E),
            0x0134 => instructions::swap_r8(self, Registers8::H),
            0x0135 => instructions::swap_r8(self, Registers8::L),
            0x0136 => instructions::swap_ar16(self, Registers16::HL),
            0x0137 => instructions::swap_r8(self, Registers8::A),

            0x0138 => instructions::srl_r8(self, Registers8::B),
            0x0139 => instructions::srl_r8(self, Registers8::C),
            0x013A => instructions::srl_r8(self, Registers8::D),
            0x013B => instructions::srl_r8(self, Registers8::E),
            0x013C => instructions::srl_r8(self, Registers8::H),
            0x013D => instructions::srl_r8(self, Registers8::L),
            0x013E => instructions::srl_ar16(self, Registers16::HL),
            0x013F => instructions::srl_r8(self, Registers8::A),

            0x0140 => instructions::bit_r8(self, 0, Registers8::B),
            0x0141 => instructions::bit_r8(self, 0, Registers8::C),
            0x0142 => instructions::bit_r8(self, 0, Registers8::D),
            0x0143 => instructions::bit_r8(self, 0, Registers8::E),
            0x0144 => instructions::bit_r8(self, 0, Registers8::H),
            0x0145 => instructions::bit_r8(self, 0, Registers8::L),
            0x0146 => instructions::bit_ar16(self, 0, Registers16::HL),
            0x0147 => instructions::bit_r8(self, 0, Registers8::A),

            0x0148 => instructions::bit_r8(self, 1, Registers8::B),
            0x0149 => instructions::bit_r8(self, 1, Registers8::C),
            0x014A => instructions::bit_r8(self, 1, Registers8::D),
            0x014B => instructions::bit_r8(self, 1, Registers8::E),
            0x014C => instructions::bit_r8(self, 1, Registers8::H),
            0x014D => instructions::bit_r8(self, 1, Registers8::L),
            0x014E => instructions::bit_ar16(self, 1, Registers16::HL),
            0x014F => instructions::bit_r8(self, 1, Registers8::A),

            0x0150 => instructions::bit_r8(self, 2, Registers8::B),
            0x0151 => instructions::bit_r8(self, 2, Registers8::C),
            0x0152 => instructions::bit_r8(self, 2, Registers8::D),
            0x0153 => instructions::bit_r8(self, 2, Registers8::E),
            0x0154 => instructions::bit_r8(self, 2, Registers8::H),
            0x0155 => instructions::bit_r8(self, 2, Registers8::L),
            0x0156 => instructions::bit_ar16(self, 2, Registers16::HL),
            0x0157 => instructions::bit_r8(self, 2, Registers8::A),

            0x0158 => instructions::bit_r8(self, 3, Registers8::B),
            0x0159 => instructions::bit_r8(self, 3, Registers8::C),
            0x015A => instructions::bit_r8(self, 3, Registers8::D),
            0x015B => instructions::bit_r8(self, 3, Registers8::E),
            0x015C => instructions::bit_r8(self, 3, Registers8::H),
            0x015D => instructions::bit_r8(self, 3, Registers8::L),
            0x015E => instructions::bit_ar16(self, 3, Registers16::HL),
            0x015F => instructions::bit_r8(self, 3, Registers8::A),

            0x0160 => instructions::bit_r8(self, 4, Registers8::B),
            0x0161 => instructions::bit_r8(self, 4, Registers8::C),
            0x0162 => instructions::bit_r8(self, 4, Registers8::D),
            0x0163 => instructions::bit_r8(self, 4, Registers8::E),
            0x0164 => instructions::bit_r8(self, 4, Registers8::H),
            0x0165 => instructions::bit_r8(self, 4, Registers8::L),
            0x0166 => instructions::bit_ar16(self, 4, Registers16::HL),
            0x0167 => instructions::bit_r8(self, 4, Registers8::A),

            0x0168 => instructions::bit_r8(self, 5, Registers8::B),
            0x0169 => instructions::bit_r8(self, 5, Registers8::C),
            0x016A => instructions::bit_r8(self, 5, Registers8::D),
            0x016B => instructions::bit_r8(self, 5, Registers8::E),
            0x016C => instructions::bit_r8(self, 5, Registers8::H),
            0x016D => instructions::bit_r8(self, 5, Registers8::L),
            0x016E => instructions::bit_ar16(self, 5, Registers16::HL),
            0x016F => instructions::bit_r8(self, 5, Registers8::A),

            0x0170 => instructions::bit_r8(self, 6, Registers8::B),
            0x0171 => instructions::bit_r8(self, 6, Registers8::C),
            0x0172 => instructions::bit_r8(self, 6, Registers8::D),
            0x0173 => instructions::bit_r8(self, 6, Registers8::E),
            0x0174 => instructions::bit_r8(self, 6, Registers8::H),
            0x0175 => instructions::bit_r8(self, 6, Registers8::L),
            0x0176 => instructions::bit_ar16(self, 6, Registers16::HL),
            0x0177 => instructions::bit_r8(self, 6, Registers8::A),

            0x0178 => instructions::bit_r8(self, 7, Registers8::B),
            0x0179 => instructions::bit_r8(self, 7, Registers8::C),
            0x017A => instructions::bit_r8(self, 7, Registers8::D),
            0x017B => instructions::bit_r8(self, 7, Registers8::E),
            0x017C => instructions::bit_r8(self, 7, Registers8::H),
            0x017D => instructions::bit_r8(self, 7, Registers8::L),
            0x017E => instructions::bit_ar16(self, 7, Registers16::HL),
            0x017F => instructions::bit_r8(self, 7, Registers8::A),

            0x0180 => instructions::res_r8(self, 0, Registers8::B),
            0x0181 => instructions::res_r8(self, 0, Registers8::C),
            0x0182 => instructions::res_r8(self, 0, Registers8::D),
            0x0183 => instructions::res_r8(self, 0, Registers8::E),
            0x0184 => instructions::res_r8(self, 0, Registers8::H),
            0x0185 => instructions::res_r8(self, 0, Registers8::L),
            0x0186 => instructions::res_ar16(self, 0, Registers16::HL),
            0x0187 => instructions::res_r8(self, 0, Registers8::A),

            0x0188 => instructions::res_r8(self, 1, Registers8::B),
            0x0189 => instructions::res_r8(self, 1, Registers8::C),
            0x018A => instructions::res_r8(self, 1, Registers8::D),
            0x018B => instructions::res_r8(self, 1, Registers8::E),
            0x018C => instructions::res_r8(self, 1, Registers8::H),
            0x018D => instructions::res_r8(self, 1, Registers8::L),
            0x018E => instructions::res_ar16(self, 1, Registers16::HL),
            0x018F => instructions::res_r8(self, 1, Registers8::A),

            0x0190 => instructions::res_r8(self, 2, Registers8::B),
            0x0191 => instructions::res_r8(self, 2, Registers8::C),
            0x0192 => instructions::res_r8(self, 2, Registers8::D),
            0x0193 => instructions::res_r8(self, 2, Registers8::E),
            0x0194 => instructions::res_r8(self, 2, Registers8::H),
            0x0195 => instructions::res_r8(self, 2, Registers8::L),
            0x0196 => instructions::res_ar16(self, 2, Registers16::HL),
            0x0197 => instructions::res_r8(self, 2, Registers8::A),

            0x0198 => instructions::res_r8(self, 3, Registers8::B),
            0x0199 => instructions::res_r8(self, 3, Registers8::C),
            0x019A => instructions::res_r8(self, 3, Registers8::D),
            0x019B => instructions::res_r8(self, 3, Registers8::E),
            0x019C => instructions::res_r8(self, 3, Registers8::H),
            0x019D => instructions::res_r8(self, 3, Registers8::L),
            0x019E => instructions::res_ar16(self, 3, Registers16::HL),
            0x019F => instructions::res_r8(self, 3, Registers8::A),

            0x01A0 => instructions::res_r8(self, 4, Registers8::B),
            0x01A1 => instructions::res_r8(self, 4, Registers8::C),
            0x01A2 => instructions::res_r8(self, 4, Registers8::D),
            0x01A3 => instructions::res_r8(self, 4, Registers8::E),
            0x01A4 => instructions::res_r8(self, 4, Registers8::H),
            0x01A5 => instructions::res_r8(self, 4, Registers8::L),
            0x01A6 => instructions::res_ar16(self, 4, Registers16::HL),
            0x01A7 => instructions::res_r8(self, 4, Registers8::A),

            0x01A8 => instructions::res_r8(self, 5, Registers8::B),
            0x01A9 => instructions::res_r8(self, 5, Registers8::C),
            0x01AA => instructions::res_r8(self, 5, Registers8::D),
            0x01AB => instructions::res_r8(self, 5, Registers8::E),
            0x01AC => instructions::res_r8(self, 5, Registers8::H),
            0x01AD => instructions::res_r8(self, 5, Registers8::L),
            0x01AE => instructions::res_ar16(self, 5, Registers16::HL),
            0x01AF => instructions::res_r8(self, 5, Registers8::A),

            0x01B0 => instructions::res_r8(self, 6, Registers8::B),
            0x01B1 => instructions::res_r8(self, 6, Registers8::C),
            0x01B2 => instructions::res_r8(self, 6, Registers8::D),
            0x01B3 => instructions::res_r8(self, 6, Registers8::E),
            0x01B4 => instructions::res_r8(self, 6, Registers8::H),
            0x01B5 => instructions::res_r8(self, 6, Registers8::L),
            0x01B6 => instructions::res_ar16(self, 6, Registers16::HL),
            0x01B7 => instructions::res_r8(self, 6, Registers8::A),

            0x01B8 => instructions::res_r8(self, 7, Registers8::B),
            0x01B9 => instructions::res_r8(self, 7, Registers8::C),
            0x01BA => instructions::res_r8(self, 7, Registers8::D),
            0x01BB => instructions::res_r8(self, 7, Registers8::E),
            0x01BC => instructions::res_r8(self, 7, Registers8::H),
            0x01BD => instructions::res_r8(self, 7, Registers8::L),
            0x01BE => instructions::res_ar16(self, 7, Registers16::HL),
            0x01BF => instructions::res_r8(self, 7, Registers8::A),

            0x01C0 => instructions::set_r8(self, 0, Registers8::B),
            0x01C1 => instructions::set_r8(self, 0, Registers8::C),
            0x01C2 => instructions::set_r8(self, 0, Registers8::D),
            0x01C3 => instructions::set_r8(self, 0, Registers8::E),
            0x01C4 => instructions::set_r8(self, 0, Registers8::H),
            0x01C5 => instructions::set_r8(self, 0, Registers8::L),
            0x01C6 => instructions::set_ar16(self, 0, Registers16::HL),
            0x01C7 => instructions::set_r8(self, 0, Registers8::A),

            0x01C8 => instructions::set_r8(self, 1, Registers8::B),
            0x01C9 => instructions::set_r8(self, 1, Registers8::C),
            0x01CA => instructions::set_r8(self, 1, Registers8::D),
            0x01CB => instructions::set_r8(self, 1, Registers8::E),
            0x01CC => instructions::set_r8(self, 1, Registers8::H),
            0x01CD => instructions::set_r8(self, 1, Registers8::L),
            0x01CE => instructions::set_ar16(self, 1, Registers16::HL),
            0x01CF => instructions::set_r8(self, 1, Registers8::A),

            0x01D0 => instructions::set_r8(self, 2, Registers8::B),
            0x01D1 => instructions::set_r8(self, 2, Registers8::C),
            0x01D2 => instructions::set_r8(self, 2, Registers8::D),
            0x01D3 => instructions::set_r8(self, 2, Registers8::E),
            0x01D4 => instructions::set_r8(self, 2, Registers8::H),
            0x01D5 => instructions::set_r8(self, 2, Registers8::L),
            0x01D6 => instructions::set_ar16(self, 2, Registers16::HL),
            0x01D7 => instructions::set_r8(self, 2, Registers8::A),

            0x01D8 => instructions::set_r8(self, 3, Registers8::B),
            0x01D9 => instructions::set_r8(self, 3, Registers8::C),
            0x01DA => instructions::set_r8(self, 3, Registers8::D),
            0x01DB => instructions::set_r8(self, 3, Registers8::E),
            0x01DC => instructions::set_r8(self, 3, Registers8::H),
            0x01DD => instructions::set_r8(self, 3, Registers8::L),
            0x01DE => instructions::set_ar16(self, 3, Registers16::HL),
            0x01DF => instructions::set_r8(self, 3, Registers8::A),

            0x01E0 => instructions::set_r8(self, 4, Registers8::B),
            0x01E1 => instructions::set_r8(self, 4, Registers8::C),
            0x01E2 => instructions::set_r8(self, 4, Registers8::D),
            0x01E3 => instructions::set_r8(self, 4, Registers8::E),
            0x01E4 => instructions::set_r8(self, 4, Registers8::H),
            0x01E5 => instructions::set_r8(self, 4, Registers8::L),
            0x01E6 => instructions::set_ar16(self, 4, Registers16::HL),
            0x01E7 => instructions::set_r8(self, 4, Registers8::A),

            0x01E8 => instructions::set_r8(self, 5, Registers8::B),
            0x01E9 => instructions::set_r8(self, 5, Registers8::C),
            0x01EA => instructions::set_r8(self, 5, Registers8::D),
            0x01EB => instructions::set_r8(self, 5, Registers8::E),
            0x01EC => instructions::set_r8(self, 5, Registers8::H),
            0x01ED => instructions::set_r8(self, 5, Registers8::L),
            0x01EE => instructions::set_ar16(self, 5, Registers16::HL),
            0x01EF => instructions::set_r8(self, 5, Registers8::A),

            0x01F0 => instructions::set_r8(self, 6, Registers8::B),
            0x01F1 => instructions::set_r8(self, 6, Registers8::C),
            0x01F2 => instructions::set_r8(self, 6, Registers8::D),
            0x01F3 => instructions::set_r8(self, 6, Registers8::E),
            0x01F4 => instructions::set_r8(self, 6, Registers8::H),
            0x01F5 => instructions::set_r8(self, 6, Registers8::L),
            0x01F6 => instructions::set_ar16(self, 6, Registers16::HL),
            0x01F7 => instructions::set_r8(self, 6, Registers8::A),

            0x01F8 => instructions::set_r8(self, 7, Registers8::B),
            0x01F9 => instructions::set_r8(self, 7, Registers8::C),
            0x01FA => instructions::set_r8(self, 7, Registers8::D),
            0x01FB => instructions::set_r8(self, 7, Registers8::E),
            0x01FC => instructions::set_r8(self, 7, Registers8::H),
            0x01FD => instructions::set_r8(self, 7, Registers8::L),
            0x01FE => instructions::set_ar16(self, 7, Registers16::HL),
            0x01FF => instructions::set_r8(self, 7, Registers8::A),

            _ => panic!("not implemented"),
        }
    }
}

