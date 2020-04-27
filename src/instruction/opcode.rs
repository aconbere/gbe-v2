use crate::instruction;
use crate::instruction::{RstFlag, JumpFlag, Instruction};

use crate::register::{Registers8, Registers16};

pub struct Fetcher {
    instructions: Vec<Instruction>
}

impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {
            instructions: build_instructions(),
        }
    }

    pub fn fetch(&self, opcode: u16) -> Option<&Instruction> {
        self.instructions.get(opcode as usize)
    }
}

pub fn build_instructions() -> Vec<Instruction> {
    let mut vec = Vec::with_capacity(512);

    vec[0x0000] = instruction::nop();
    vec[0x0001] = instruction::ld_r16_n16(Registers16::BC);
    vec[0x0002] = instruction::ld_ar16_r8(Registers16::BC, Registers8::A);
    vec[0x0003] = instruction::inc_r16(Registers16::BC);
    vec[0x0004] = instruction::inc_r8(Registers8::B);
    vec[0x0005] = instruction::dec_r8(Registers8::B);
    vec[0x0006] = instruction::ld_r8_n8(Registers8::B);
    vec[0x0007] = instruction::rlca();
    vec[0x0008] = instruction::ld_an16_r16(Registers16::SP);
    vec[0x0009] = instruction::add_r16_r16(Registers16::HL, Registers16::BC);
    vec[0x000A] = instruction::ld_r8_ar16(Registers8::A, Registers16::BC);
    vec[0x000B] = instruction::dec_r16(Registers16::BC);
    vec[0x000C] = instruction::inc_r8(Registers8::C);
    vec[0x000D] = instruction::dec_r8(Registers8::C);
    vec[0x000E] = instruction::ld_r8_n8(Registers8::C);
    vec[0x000F] = instruction::rrca();

    vec[0x0010] = instruction::stop();
    vec[0x0011] = instruction::ld_r16_n16(Registers16::DE);
    vec[0x0012] = instruction::ld_ar16_r8(Registers16::DE, Registers8::A);
    vec[0x0013] = instruction::inc_r16(Registers16::DE);
    vec[0x0014] = instruction::inc_r8(Registers8::D);
    vec[0x0015] = instruction::dec_r8(Registers8::D);
    vec[0x0016] = instruction::ld_r8_n8(Registers8::D);
    vec[0x0017] = instruction::rla();
    vec[0x0018] = instruction::jr_n8();
    vec[0x0019] = instruction::add_r16_r16(Registers16::HL, Registers16::DE);
    vec[0x001A] = instruction::ld_r8_ar16(Registers8::A, Registers16::DE);
    vec[0x001B] = instruction::dec_r16(Registers16::DE);
    vec[0x001C] = instruction::inc_r8(Registers8::E);
    vec[0x001D] = instruction::dec_r8(Registers8::E);
    vec[0x001E] = instruction::ld_r8_n8(Registers8::E);
    vec[0x001F] = instruction::rra();

    vec[0x0020] = instruction::jr_f_n8(JumpFlag::NZ);
    vec[0x0021] = instruction::ld_r16_n16(Registers16::HL);
    vec[0x0022] = instruction::ldi_ar16_r8(Registers16::HL, Registers8::A);
    vec[0x0023] = instruction::inc_r16(Registers16::HL);
    vec[0x0024] = instruction::inc_r8(Registers8::H);
    vec[0x0025] = instruction::dec_r8(Registers8::H);
    vec[0x0026] = instruction::ld_r8_n8(Registers8::H);
    vec[0x0027] = instruction::daa();
    vec[0x0028] = instruction::jr_f_n8(JumpFlag::Z);
    vec[0x0029] = instruction::add_r16_r16(Registers16::HL, Registers16::HL);
    vec[0x002A] = instruction::ldi_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x002B] = instruction::dec_r16(Registers16::HL);
    vec[0x002C] = instruction::inc_r8(Registers8::L);
    vec[0x002D] = instruction::dec_r8(Registers8::L);
    vec[0x002E] = instruction::ld_r8_n8(Registers8::L);
    vec[0x002F] = instruction::cpl();

    vec[0x0030] = instruction::jr_f_n8(JumpFlag::NC);
    vec[0x0031] = instruction::ld_r16_n16(Registers16::SP);
    vec[0x0032] = instruction::ldd_ar16_r8(Registers16::HL, Registers8::A);
    vec[0x0033] = instruction::inc_r16(Registers16::SP);
    vec[0x0034] = instruction::inc_ar16(Registers16::HL);
    vec[0x0035] = instruction::dec_ar16(Registers16::HL);
    vec[0x0036] = instruction::ld_ar16_n8(Registers16::HL);
    vec[0x0037] = instruction::scf();
    vec[0x0038] = instruction::jr_f_n8(JumpFlag::C);
    vec[0x0039] = instruction::add_r16_r16(Registers16::HL, Registers16::SP);
    vec[0x003A] = instruction::ldd_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x003B] = instruction::dec_r16(Registers16::SP);
    vec[0x003C] = instruction::inc_r8(Registers8::A);
    vec[0x003D] = instruction::dec_r8(Registers8::A);
    vec[0x003E] = instruction::ld_r8_n8(Registers8::A);
    vec[0x003F] = instruction::ccf();

    vec[0x0040] = instruction::ld_r8_r8(Registers8::B, Registers8::B);
    vec[0x0041] = instruction::ld_r8_r8(Registers8::B, Registers8::C);
    vec[0x0042] = instruction::ld_r8_r8(Registers8::B, Registers8::D);
    vec[0x0043] = instruction::ld_r8_r8(Registers8::B, Registers8::E);
    vec[0x0044] = instruction::ld_r8_r8(Registers8::B, Registers8::H);
    vec[0x0045] = instruction::ld_r8_r8(Registers8::B, Registers8::L);
    vec[0x0046] = instruction::ld_r8_ar16(Registers8::B, Registers16::HL);
    vec[0x0047] = instruction::ld_r8_r8(Registers8::B, Registers8::A);

    vec[0x0048] = instruction::ld_r8_r8(Registers8::C, Registers8::B);
    vec[0x0049] = instruction::ld_r8_r8(Registers8::C, Registers8::C);
    vec[0x004A] = instruction::ld_r8_r8(Registers8::C, Registers8::D);
    vec[0x004B] = instruction::ld_r8_r8(Registers8::C, Registers8::E);
    vec[0x004C] = instruction::ld_r8_r8(Registers8::C, Registers8::H);
    vec[0x004D] = instruction::ld_r8_r8(Registers8::C, Registers8::L);
    vec[0x004E] = instruction::ld_r8_ar16(Registers8::C, Registers16::HL);
    vec[0x004F] = instruction::ld_r8_r8(Registers8::C, Registers8::A);

    vec[0x0050] = instruction::ld_r8_r8(Registers8::D, Registers8::B);
    vec[0x0051] = instruction::ld_r8_r8(Registers8::D, Registers8::C);
    vec[0x0052] = instruction::ld_r8_r8(Registers8::D, Registers8::D);
    vec[0x0053] = instruction::ld_r8_r8(Registers8::D, Registers8::E);
    vec[0x0054] = instruction::ld_r8_r8(Registers8::D, Registers8::H);
    vec[0x0055] = instruction::ld_r8_r8(Registers8::D, Registers8::L);
    vec[0x0056] = instruction::ld_r8_ar16(Registers8::D, Registers16::HL);
    vec[0x0057] = instruction::ld_r8_r8(Registers8::D, Registers8::A);

    vec[0x0058] = instruction::ld_r8_r8(Registers8::E, Registers8::B);
    vec[0x0059] = instruction::ld_r8_r8(Registers8::E, Registers8::C);
    vec[0x005A] = instruction::ld_r8_r8(Registers8::E, Registers8::D);
    vec[0x005B] = instruction::ld_r8_r8(Registers8::E, Registers8::E);
    vec[0x005C] = instruction::ld_r8_r8(Registers8::E, Registers8::H);
    vec[0x005D] = instruction::ld_r8_r8(Registers8::E, Registers8::L);
    vec[0x005E] = instruction::ld_r8_ar16(Registers8::E, Registers16::HL);
    vec[0x005F] = instruction::ld_r8_r8(Registers8::E, Registers8::A);

    vec[0x0060] = instruction::ld_r8_r8(Registers8::H, Registers8::B);
    vec[0x0061] = instruction::ld_r8_r8(Registers8::H, Registers8::C);
    vec[0x0062] = instruction::ld_r8_r8(Registers8::H, Registers8::D);
    vec[0x0063] = instruction::ld_r8_r8(Registers8::H, Registers8::E);
    vec[0x0064] = instruction::ld_r8_r8(Registers8::H, Registers8::H);
    vec[0x0065] = instruction::ld_r8_r8(Registers8::H, Registers8::L);
    vec[0x0066] = instruction::ld_r8_ar16(Registers8::H, Registers16::HL);
    vec[0x0067] = instruction::ld_r8_r8(Registers8::H, Registers8::A);

    vec[0x0068] = instruction::ld_r8_r8(Registers8::L, Registers8::B);
    vec[0x0069] = instruction::ld_r8_r8(Registers8::L, Registers8::C);
    vec[0x006A] = instruction::ld_r8_r8(Registers8::L, Registers8::D);
    vec[0x006B] = instruction::ld_r8_r8(Registers8::L, Registers8::E);
    vec[0x006C] = instruction::ld_r8_r8(Registers8::L, Registers8::H);
    vec[0x006D] = instruction::ld_r8_r8(Registers8::L, Registers8::L);
    vec[0x006E] = instruction::ld_r8_ar16(Registers8::L, Registers16::HL);
    vec[0x006F] = instruction::ld_r8_r8(Registers8::L, Registers8::A);

    vec[0x0070] = instruction::ld_ar16_r8(Registers16::HL, Registers8::B);
    vec[0x0071] = instruction::ld_ar16_r8(Registers16::HL, Registers8::C);
    vec[0x0072] = instruction::ld_ar16_r8(Registers16::HL, Registers8::D);
    vec[0x0073] = instruction::ld_ar16_r8(Registers16::HL, Registers8::E);
    vec[0x0074] = instruction::ld_ar16_r8(Registers16::HL, Registers8::H);
    vec[0x0075] = instruction::ld_ar16_r8(Registers16::HL, Registers8::L);
    vec[0x0076] = instruction::halt();
    vec[0x0077] = instruction::ld_ar16_r8(Registers16::HL, Registers8::A);

    vec[0x0078] = instruction::ld_r8_r8(Registers8::A, Registers8::B);
    vec[0x0079] = instruction::ld_r8_r8(Registers8::A, Registers8::C);
    vec[0x007A] = instruction::ld_r8_r8(Registers8::A, Registers8::D);
    vec[0x007B] = instruction::ld_r8_r8(Registers8::A, Registers8::E);
    vec[0x007C] = instruction::ld_r8_r8(Registers8::A, Registers8::H);
    vec[0x007D] = instruction::ld_r8_r8(Registers8::A, Registers8::L);
    vec[0x007E] = instruction::ld_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x007F] = instruction::ld_r8_r8(Registers8::A, Registers8::A);

    vec[0x0080] = instruction::add_r8_r8(Registers8::A, Registers8::B);
    vec[0x0081] = instruction::add_r8_r8(Registers8::A, Registers8::C);
    vec[0x0082] = instruction::add_r8_r8(Registers8::A, Registers8::D);
    vec[0x0083] = instruction::add_r8_r8(Registers8::A, Registers8::E);
    vec[0x0084] = instruction::add_r8_r8(Registers8::A, Registers8::H);
    vec[0x0085] = instruction::add_r8_r8(Registers8::A, Registers8::L);
    vec[0x0086] = instruction::add_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x0087] = instruction::add_r8_r8(Registers8::A, Registers8::A);

    vec[0x0088] = instruction::adc_r8_r8(Registers8::A, Registers8::B);
    vec[0x0089] = instruction::adc_r8_r8(Registers8::A, Registers8::C);
    vec[0x008A] = instruction::adc_r8_r8(Registers8::A, Registers8::D);
    vec[0x008B] = instruction::adc_r8_r8(Registers8::A, Registers8::E);
    vec[0x008C] = instruction::adc_r8_r8(Registers8::A, Registers8::H);
    vec[0x008D] = instruction::adc_r8_r8(Registers8::A, Registers8::L);
    vec[0x008E] = instruction::adc_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x008F] = instruction::adc_r8_r8(Registers8::A, Registers8::A);

    vec[0x0090] = instruction::sub_r8_r8(Registers8::A, Registers8::B);
    vec[0x0091] = instruction::sub_r8_r8(Registers8::A, Registers8::C);
    vec[0x0092] = instruction::sub_r8_r8(Registers8::A, Registers8::D);
    vec[0x0093] = instruction::sub_r8_r8(Registers8::A, Registers8::E);
    vec[0x0094] = instruction::sub_r8_r8(Registers8::A, Registers8::H);
    vec[0x0095] = instruction::sub_r8_r8(Registers8::A, Registers8::L);
    vec[0x0096] = instruction::sub_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x0097] = instruction::sub_r8_r8(Registers8::A, Registers8::A);

    vec[0x0098] = instruction::sbc_r8_r8(Registers8::A, Registers8::B);
    vec[0x0099] = instruction::sbc_r8_r8(Registers8::A, Registers8::C);
    vec[0x009A] = instruction::sbc_r8_r8(Registers8::A, Registers8::D);
    vec[0x009B] = instruction::sbc_r8_r8(Registers8::A, Registers8::E);
    vec[0x009C] = instruction::sbc_r8_r8(Registers8::A, Registers8::H);
    vec[0x009D] = instruction::sbc_r8_r8(Registers8::A, Registers8::L);
    vec[0x009E] = instruction::sbc_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x009F] = instruction::sbc_r8_r8(Registers8::A, Registers8::A);

    vec[0x00A0] = instruction::and_r8_r8(Registers8::A, Registers8::B);
    vec[0x00A1] = instruction::and_r8_r8(Registers8::A, Registers8::C);
    vec[0x00A2] = instruction::and_r8_r8(Registers8::A, Registers8::D);
    vec[0x00A3] = instruction::and_r8_r8(Registers8::A, Registers8::E);
    vec[0x00A4] = instruction::and_r8_r8(Registers8::A, Registers8::H);
    vec[0x00A5] = instruction::and_r8_r8(Registers8::A, Registers8::L);
    vec[0x00A6] = instruction::and_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x00A7] = instruction::and_r8_r8(Registers8::A, Registers8::A);

    vec[0x00A8] = instruction::xor_r8_r8(Registers8::A, Registers8::B);
    vec[0x00A9] = instruction::xor_r8_r8(Registers8::A, Registers8::C);
    vec[0x00AA] = instruction::xor_r8_r8(Registers8::A, Registers8::D);
    vec[0x00AB] = instruction::xor_r8_r8(Registers8::A, Registers8::E);
    vec[0x00AC] = instruction::xor_r8_r8(Registers8::A, Registers8::H);
    vec[0x00AD] = instruction::xor_r8_r8(Registers8::A, Registers8::L);
    vec[0x00AE] = instruction::xor_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x00AF] = instruction::xor_r8_r8(Registers8::A, Registers8::A);

    vec[0x00B0] = instruction::or_r8_r8(Registers8::A, Registers8::B);
    vec[0x00B1] = instruction::or_r8_r8(Registers8::A, Registers8::C);
    vec[0x00B2] = instruction::or_r8_r8(Registers8::A, Registers8::D);
    vec[0x00B3] = instruction::or_r8_r8(Registers8::A, Registers8::E);
    vec[0x00B4] = instruction::or_r8_r8(Registers8::A, Registers8::H);
    vec[0x00B5] = instruction::or_r8_r8(Registers8::A, Registers8::L);
    vec[0x00B6] = instruction::or_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x00B7] = instruction::or_r8_r8(Registers8::A, Registers8::A);

    vec[0x00B8] = instruction::cp_r8_r8(Registers8::A, Registers8::B);
    vec[0x00B9] = instruction::cp_r8_r8(Registers8::A, Registers8::C);
    vec[0x00BA] = instruction::cp_r8_r8(Registers8::A, Registers8::D);
    vec[0x00BB] = instruction::cp_r8_r8(Registers8::A, Registers8::E);
    vec[0x00BC] = instruction::cp_r8_r8(Registers8::A, Registers8::H);
    vec[0x00BD] = instruction::cp_r8_r8(Registers8::A, Registers8::L);
    vec[0x00BE] = instruction::cp_r8_ar16(Registers8::A, Registers16::HL);
    vec[0x00BF] = instruction::cp_r8_r8(Registers8::A, Registers8::A);

    vec[0x00C0] = instruction::ret_f(JumpFlag::NZ);
    vec[0x00C1] = instruction::pop_r16(Registers16::BC);
    vec[0x00C2] = instruction::jp_f_n16(JumpFlag::NZ);
    vec[0x00C3] = instruction::jp_n16();
    vec[0x00C4] = instruction::call_f_n16(JumpFlag::NZ);
    vec[0x00C5] = instruction::push_r16(Registers16::BC);
    vec[0x00C6] = instruction::add_r8_n8(Registers8::A);
    vec[0x00C7] = instruction::rst_f(RstFlag::H00);

    vec[0x00C8] = instruction::ret_f(JumpFlag::Z);
    vec[0x00C9] = instruction::ret();
    vec[0x00CA] = instruction::jp_f_n16(JumpFlag::Z);
    vec[0x00CB] = instruction::illegal_opcode(0x00CB);
    vec[0x00CC] = instruction::call_f_n16(JumpFlag::Z);
    vec[0x00CD] = instruction::call_n16();
    vec[0x00CE] = instruction::adc_r8_n8(Registers8::A);
    vec[0x00CF] = instruction::rst_f(RstFlag::H08);

    vec[0x00D0] = instruction::ret_f(JumpFlag::NC);
    vec[0x00D1] = instruction::pop_r16(Registers16::DE);
    vec[0x00D2] = instruction::jp_f_n16(JumpFlag::NC);
    vec[0x00D3] = instruction::illegal_opcode(0x00D3);
    vec[0x00D4] = instruction::call_f_n16(JumpFlag::NC);
    vec[0x00D5] = instruction::push_r16(Registers16::DE);
    vec[0x00D6] = instruction::sub_r8_n8(Registers8::A);
    vec[0x00D7] = instruction::rst_f(RstFlag::H10);

    vec[0x00D8] = instruction::ret_f(JumpFlag::C);
    vec[0x00D9] = instruction::reti();
    vec[0x00DA] = instruction::jp_f_n16(JumpFlag::C);
    vec[0x00DB] = instruction::illegal_opcode(0x00DB);
    vec[0x00DC] = instruction::call_f_n16(JumpFlag::C);
    vec[0x00DD] = instruction::illegal_opcode(0x00DD);
    vec[0x00DE] = instruction::sbc_r8_n8(Registers8::A);
    vec[0x00DF] = instruction::rst_f(RstFlag::H18);

    vec[0x00E0] = instruction::ldh_an8_r8(Registers8::A);
    vec[0x00E1] = instruction::pop_r16(Registers16::HL);
    vec[0x00E2] = instruction::ldc_ar8_r8(Registers8::C, Registers8::A);
    vec[0x00E3] = instruction::illegal_opcode(0x00E3);
    vec[0x00E4] = instruction::illegal_opcode(0x00E4);
    vec[0x00E5] = instruction::push_r16(Registers16::HL);
    vec[0x00E6] = instruction::and_r8_n8(Registers8::A);
    vec[0x00E7] = instruction::rst_f(RstFlag::H20);

    vec[0x00E8] = instruction::add_r16_n8(Registers16::SP);
    vec[0x00E9] = instruction::jp_r16(Registers16::HL);
    vec[0x00EA] = instruction::ld_an16_r8(Registers8::A);
    vec[0x00EB] = instruction::illegal_opcode(0x00EB);
    vec[0x00EC] = instruction::illegal_opcode(0x00EC);
    vec[0x00ED] = instruction::illegal_opcode(0x00ED);
    vec[0x00EE] = instruction::xor_r8_n8(Registers8::A);
    vec[0x00EF] = instruction::rst_f(RstFlag::H28);

    vec[0x00F0] = instruction::ldh_r8_an8(Registers8::A);
    vec[0x00F1] = instruction::pop_r16(Registers16::AF);
    vec[0x00F2] = instruction::ldc_r8_ar8(Registers8::A, Registers8::C);
    vec[0x00F3] = instruction::di();
    vec[0x00F4] = instruction::illegal_opcode(0x00F4);
    vec[0x00F5] = instruction::push_r16(Registers16::AF);
    vec[0x00F6] = instruction::or_r8_n8(Registers8::A);
    vec[0x00F7] = instruction::rst_f(RstFlag::H30);

    vec[0x00F8] = instruction::ld_r16_spn8(Registers16::HL);
    vec[0x00F9] = instruction::ld_r16_r16(Registers16::SP, Registers16::HL);
    vec[0x00FA] = instruction::ld_r8_an16(Registers8::A);
    vec[0x00FB] = instruction::ei();
    vec[0x00FC] = instruction::illegal_opcode(0x00FC);
    vec[0x00FD] = instruction::illegal_opcode(0x00FD);
    vec[0x00FE] = instruction::cp_r8_n8(Registers8::A);
    vec[0x00FF] = instruction::rst_f(RstFlag::H38);

    // Prefix instruction::CB

    vec[0xCB00] = instruction::rlc_r8(Registers8::B);
    vec[0xCB01] = instruction::rlc_r8(Registers8::C);
    vec[0xCB02] = instruction::rlc_r8(Registers8::D);
    vec[0xCB03] = instruction::rlc_r8(Registers8::E);
    vec[0xCB04] = instruction::rlc_r8(Registers8::H);
    vec[0xCB05] = instruction::rlc_r8(Registers8::L);
    vec[0xCB06] = instruction::rlc_ar16(Registers16::HL);
    vec[0xCB07] = instruction::rlc_r8(Registers8::A);

    vec[0xCB08] = instruction::rrc_r8(Registers8::B);
    vec[0xCB09] = instruction::rrc_r8(Registers8::C);
    vec[0xCB0A] = instruction::rrc_r8(Registers8::D);
    vec[0xCB0B] = instruction::rrc_r8(Registers8::E);
    vec[0xCB0C] = instruction::rrc_r8(Registers8::H);
    vec[0xCB0D] = instruction::rrc_r8(Registers8::L);
    vec[0xCB0E] = instruction::rrc_ar16(Registers16::HL);
    vec[0xCB0F] = instruction::rrc_r8(Registers8::A);

    vec[0xCB10] = instruction::rl_r8(Registers8::B);
    vec[0xCB11] = instruction::rl_r8(Registers8::C);
    vec[0xCB12] = instruction::rl_r8(Registers8::D);
    vec[0xCB13] = instruction::rl_r8(Registers8::E);
    vec[0xCB14] = instruction::rl_r8(Registers8::H);
    vec[0xCB15] = instruction::rl_r8(Registers8::L);
    vec[0xCB16] = instruction::rl_ar16(Registers16::HL);
    vec[0xCB17] = instruction::rl_r8(Registers8::A);

    vec[0xCB18] = instruction::rr_r8(Registers8::B);
    vec[0xCB19] = instruction::rr_r8(Registers8::C);
    vec[0xCB1A] = instruction::rr_r8(Registers8::D);
    vec[0xCB1B] = instruction::rr_r8(Registers8::E);
    vec[0xCB1C] = instruction::rr_r8(Registers8::H);
    vec[0xCB1D] = instruction::rr_r8(Registers8::L);
    vec[0xCB1E] = instruction::rr_ar16(Registers16::HL);
    vec[0xCB1F] = instruction::rr_r8(Registers8::A);

    vec[0xCB20] = instruction::sla_r8(Registers8::B);
    vec[0xCB21] = instruction::sla_r8(Registers8::C);
    vec[0xCB22] = instruction::sla_r8(Registers8::D);
    vec[0xCB23] = instruction::sla_r8(Registers8::E);
    vec[0xCB24] = instruction::sla_r8(Registers8::H);
    vec[0xCB25] = instruction::sla_r8(Registers8::L);
    vec[0xCB26] = instruction::sla_ar16(Registers16::HL);
    vec[0xCB27] = instruction::sla_r8(Registers8::A);

    vec[0xCB28] = instruction::sra_r8(Registers8::B);
    vec[0xCB29] = instruction::sra_r8(Registers8::C);
    vec[0xCB2A] = instruction::sra_r8(Registers8::D);
    vec[0xCB2B] = instruction::sra_r8(Registers8::E);
    vec[0xCB2C] = instruction::sra_r8(Registers8::H);
    vec[0xCB2D] = instruction::sra_r8(Registers8::L);
    vec[0xCB2E] = instruction::sra_ar16(Registers16::HL);
    vec[0xCB2F] = instruction::sra_r8(Registers8::A);

    vec[0xCB30] = instruction::swap_r8(Registers8::B);
    vec[0xCB31] = instruction::swap_r8(Registers8::C);
    vec[0xCB32] = instruction::swap_r8(Registers8::D);
    vec[0xCB33] = instruction::swap_r8(Registers8::E);
    vec[0xCB34] = instruction::swap_r8(Registers8::H);
    vec[0xCB35] = instruction::swap_r8(Registers8::L);
    vec[0xCB36] = instruction::swap_ar16(Registers16::HL);
    vec[0xCB37] = instruction::swap_r8(Registers8::A);

    vec[0xCB38] = instruction::srl_r8(Registers8::B);
    vec[0xCB39] = instruction::srl_r8(Registers8::C);
    vec[0xCB3A] = instruction::srl_r8(Registers8::D);
    vec[0xCB3B] = instruction::srl_r8(Registers8::E);
    vec[0xCB3C] = instruction::srl_r8(Registers8::H);
    vec[0xCB3D] = instruction::srl_r8(Registers8::L);
    vec[0xCB3E] = instruction::srl_ar16(Registers16::HL);
    vec[0xCB3F] = instruction::srl_r8(Registers8::A);

    vec[0xCB40] = instruction::bit_r8(0, Registers8::B);
    vec[0xCB41] = instruction::bit_r8(0, Registers8::C);
    vec[0xCB42] = instruction::bit_r8(0, Registers8::D);
    vec[0xCB43] = instruction::bit_r8(0, Registers8::E);
    vec[0xCB44] = instruction::bit_r8(0, Registers8::H);
    vec[0xCB45] = instruction::bit_r8(0, Registers8::L);
    vec[0xCB46] = instruction::bit_ar16(0, Registers16::HL);
    vec[0xCB47] = instruction::bit_r8(0, Registers8::A);

    vec[0xCB48] = instruction::bit_r8(1, Registers8::B);
    vec[0xCB49] = instruction::bit_r8(1, Registers8::C);
    vec[0xCB4A] = instruction::bit_r8(1, Registers8::D);
    vec[0xCB4B] = instruction::bit_r8(1, Registers8::E);
    vec[0xCB4C] = instruction::bit_r8(1, Registers8::H);
    vec[0xCB4D] = instruction::bit_r8(1, Registers8::L);
    vec[0xCB4E] = instruction::bit_ar16(1, Registers16::HL);
    vec[0xCB4F] = instruction::bit_r8(1, Registers8::A);

    vec[0xCB50] = instruction::bit_r8(2, Registers8::B);
    vec[0xCB51] = instruction::bit_r8(2, Registers8::C);
    vec[0xCB52] = instruction::bit_r8(2, Registers8::D);
    vec[0xCB53] = instruction::bit_r8(2, Registers8::E);
    vec[0xCB54] = instruction::bit_r8(2, Registers8::H);
    vec[0xCB55] = instruction::bit_r8(2, Registers8::L);
    vec[0xCB56] = instruction::bit_ar16(2, Registers16::HL);
    vec[0xCB57] = instruction::bit_r8(2, Registers8::A);

    vec[0xCB58] = instruction::bit_r8(3, Registers8::B);
    vec[0xCB59] = instruction::bit_r8(3, Registers8::C);
    vec[0xCB5A] = instruction::bit_r8(3, Registers8::D);
    vec[0xCB5B] = instruction::bit_r8(3, Registers8::E);
    vec[0xCB5C] = instruction::bit_r8(3, Registers8::H);
    vec[0xCB5D] = instruction::bit_r8(3, Registers8::L);
    vec[0xCB5E] = instruction::bit_ar16(3, Registers16::HL);
    vec[0xCB5F] = instruction::bit_r8(3, Registers8::A);

    vec[0xCB60] = instruction::bit_r8(4, Registers8::B);
    vec[0xCB61] = instruction::bit_r8(4, Registers8::C);
    vec[0xCB62] = instruction::bit_r8(4, Registers8::D);
    vec[0xCB63] = instruction::bit_r8(4, Registers8::E);
    vec[0xCB64] = instruction::bit_r8(4, Registers8::H);
    vec[0xCB65] = instruction::bit_r8(4, Registers8::L);
    vec[0xCB66] = instruction::bit_ar16(4, Registers16::HL);
    vec[0xCB67] = instruction::bit_r8(4, Registers8::A);

    vec[0xCB68] = instruction::bit_r8(5, Registers8::B);
    vec[0xCB69] = instruction::bit_r8(5, Registers8::C);
    vec[0xCB6A] = instruction::bit_r8(5, Registers8::D);
    vec[0xCB6B] = instruction::bit_r8(5, Registers8::E);
    vec[0xCB6C] = instruction::bit_r8(5, Registers8::H);
    vec[0xCB6D] = instruction::bit_r8(5, Registers8::L);
    vec[0xCB6E] = instruction::bit_ar16(5, Registers16::HL);
    vec[0xCB6F] = instruction::bit_r8(5, Registers8::A);

    vec[0xCB70] = instruction::bit_r8(6, Registers8::B);
    vec[0xCB71] = instruction::bit_r8(6, Registers8::C);
    vec[0xCB72] = instruction::bit_r8(6, Registers8::D);
    vec[0xCB73] = instruction::bit_r8(6, Registers8::E);
    vec[0xCB74] = instruction::bit_r8(6, Registers8::H);
    vec[0xCB75] = instruction::bit_r8(6, Registers8::L);
    vec[0xCB76] = instruction::bit_ar16(6, Registers16::HL);
    vec[0xCB77] = instruction::bit_r8(6, Registers8::A);

    vec[0xCB78] = instruction::bit_r8(7, Registers8::B);
    vec[0xCB79] = instruction::bit_r8(7, Registers8::C);
    vec[0xCB7A] = instruction::bit_r8(7, Registers8::D);
    vec[0xCB7B] = instruction::bit_r8(7, Registers8::E);
    vec[0xCB7C] = instruction::bit_r8(7, Registers8::H);
    vec[0xCB7D] = instruction::bit_r8(7, Registers8::L);
    vec[0xCB7E] = instruction::bit_ar16(7, Registers16::HL);
    vec[0xCB7F] = instruction::bit_r8(7, Registers8::A);

    vec[0xCB80] = instruction::res_r8(0, Registers8::B);
    vec[0xCB81] = instruction::res_r8(0, Registers8::C);
    vec[0xCB82] = instruction::res_r8(0, Registers8::D);
    vec[0xCB83] = instruction::res_r8(0, Registers8::E);
    vec[0xCB84] = instruction::res_r8(0, Registers8::H);
    vec[0xCB85] = instruction::res_r8(0, Registers8::L);
    vec[0xCB86] = instruction::res_ar16(0, Registers16::HL);
    vec[0xCB87] = instruction::res_r8(0, Registers8::A);

    vec[0xCB88] = instruction::res_r8(1, Registers8::B);
    vec[0xCB89] = instruction::res_r8(1, Registers8::C);
    vec[0xCB8A] = instruction::res_r8(1, Registers8::D);
    vec[0xCB8B] = instruction::res_r8(1, Registers8::E);
    vec[0xCB8C] = instruction::res_r8(1, Registers8::H);
    vec[0xCB8D] = instruction::res_r8(1, Registers8::L);
    vec[0xCB8E] = instruction::res_ar16(1, Registers16::HL);
    vec[0xCB8F] = instruction::res_r8(1, Registers8::A);

    vec[0xCB90] = instruction::res_r8(2, Registers8::B);
    vec[0xCB91] = instruction::res_r8(2, Registers8::C);
    vec[0xCB92] = instruction::res_r8(2, Registers8::D);
    vec[0xCB93] = instruction::res_r8(2, Registers8::E);
    vec[0xCB94] = instruction::res_r8(2, Registers8::H);
    vec[0xCB95] = instruction::res_r8(2, Registers8::L);
    vec[0xCB96] = instruction::res_ar16(2, Registers16::HL);
    vec[0xCB97] = instruction::res_r8(2, Registers8::A);

    vec[0xCB98] = instruction::res_r8(3, Registers8::B);
    vec[0xCB99] = instruction::res_r8(3, Registers8::C);
    vec[0xCB9A] = instruction::res_r8(3, Registers8::D);
    vec[0xCB9B] = instruction::res_r8(3, Registers8::E);
    vec[0xCB9C] = instruction::res_r8(3, Registers8::H);
    vec[0xCB9D] = instruction::res_r8(3, Registers8::L);
    vec[0xCB9E] = instruction::res_ar16(3, Registers16::HL);
    vec[0xCB9F] = instruction::res_r8(3, Registers8::A);

    vec[0xCBA0] = instruction::res_r8(4, Registers8::B);
    vec[0xCBA1] = instruction::res_r8(4, Registers8::C);
    vec[0xCBA2] = instruction::res_r8(4, Registers8::D);
    vec[0xCBA3] = instruction::res_r8(4, Registers8::E);
    vec[0xCBA4] = instruction::res_r8(4, Registers8::H);
    vec[0xCBA5] = instruction::res_r8(4, Registers8::L);
    vec[0xCBA6] = instruction::res_ar16(4, Registers16::HL);
    vec[0xCBA7] = instruction::res_r8(4, Registers8::A);

    vec[0xCBA8] = instruction::res_r8(5, Registers8::B);
    vec[0xCBA9] = instruction::res_r8(5, Registers8::C);
    vec[0xCBAA] = instruction::res_r8(5, Registers8::D);
    vec[0xCBAB] = instruction::res_r8(5, Registers8::E);
    vec[0xCBAC] = instruction::res_r8(5, Registers8::H);
    vec[0xCBAD] = instruction::res_r8(5, Registers8::L);
    vec[0xCBAE] = instruction::res_ar16(5, Registers16::HL);
    vec[0xCBAF] = instruction::res_r8(5, Registers8::A);

    vec[0xCBB0] = instruction::res_r8(6, Registers8::B);
    vec[0xCBB1] = instruction::res_r8(6, Registers8::C);
    vec[0xCBB2] = instruction::res_r8(6, Registers8::D);
    vec[0xCBB3] = instruction::res_r8(6, Registers8::E);
    vec[0xCBB4] = instruction::res_r8(6, Registers8::H);
    vec[0xCBB5] = instruction::res_r8(6, Registers8::L);
    vec[0xCBB6] = instruction::res_ar16(6, Registers16::HL);
    vec[0xCBB7] = instruction::res_r8(6, Registers8::A);

    vec[0xCBB8] = instruction::res_r8(7, Registers8::B);
    vec[0xCBB9] = instruction::res_r8(7, Registers8::C);
    vec[0xCBBA] = instruction::res_r8(7, Registers8::D);
    vec[0xCBBB] = instruction::res_r8(7, Registers8::E);
    vec[0xCBBC] = instruction::res_r8(7, Registers8::H);
    vec[0xCBBD] = instruction::res_r8(7, Registers8::L);
    vec[0xCBBE] = instruction::res_ar16(7, Registers16::HL);
    vec[0xCBBF] = instruction::res_r8(7, Registers8::A);

    vec[0xCBC0] = instruction::set_r8(0, Registers8::B);
    vec[0xCBC1] = instruction::set_r8(0, Registers8::C);
    vec[0xCBC2] = instruction::set_r8(0, Registers8::D);
    vec[0xCBC3] = instruction::set_r8(0, Registers8::E);
    vec[0xCBC4] = instruction::set_r8(0, Registers8::H);
    vec[0xCBC5] = instruction::set_r8(0, Registers8::L);
    vec[0xCBC6] = instruction::set_ar16(0, Registers16::HL);
    vec[0xCBC7] = instruction::set_r8(0, Registers8::A);

    vec[0xCBC8] = instruction::set_r8(1, Registers8::B);
    vec[0xCBC9] = instruction::set_r8(1, Registers8::C);
    vec[0xCBCA] = instruction::set_r8(1, Registers8::D);
    vec[0xCBCB] = instruction::set_r8(1, Registers8::E);
    vec[0xCBCC] = instruction::set_r8(1, Registers8::H);
    vec[0xCBCD] = instruction::set_r8(1, Registers8::L);
    vec[0xCBCE] = instruction::set_ar16(1, Registers16::HL);
    vec[0xCBCF] = instruction::set_r8(1, Registers8::A);

    vec[0xCBD0] = instruction::set_r8(2, Registers8::B);
    vec[0xCBD1] = instruction::set_r8(2, Registers8::C);
    vec[0xCBD2] = instruction::set_r8(2, Registers8::D);
    vec[0xCBD3] = instruction::set_r8(2, Registers8::E);
    vec[0xCBD4] = instruction::set_r8(2, Registers8::H);
    vec[0xCBD5] = instruction::set_r8(2, Registers8::L);
    vec[0xCBD6] = instruction::set_ar16(2, Registers16::HL);
    vec[0xCBD7] = instruction::set_r8(2, Registers8::A);

    vec[0xCBD8] = instruction::set_r8(3, Registers8::B);
    vec[0xCBD9] = instruction::set_r8(3, Registers8::C);
    vec[0xCBDA] = instruction::set_r8(3, Registers8::D);
    vec[0xCBDB] = instruction::set_r8(3, Registers8::E);
    vec[0xCBDC] = instruction::set_r8(3, Registers8::H);
    vec[0xCBDD] = instruction::set_r8(3, Registers8::L);
    vec[0xCBDE] = instruction::set_ar16(3, Registers16::HL);
    vec[0xCBDF] = instruction::set_r8(3, Registers8::A);

    vec[0xCBE0] = instruction::set_r8(4, Registers8::B);
    vec[0xCBE1] = instruction::set_r8(4, Registers8::C);
    vec[0xCBE2] = instruction::set_r8(4, Registers8::D);
    vec[0xCBE3] = instruction::set_r8(4, Registers8::E);
    vec[0xCBE4] = instruction::set_r8(4, Registers8::H);
    vec[0xCBE5] = instruction::set_r8(4, Registers8::L);
    vec[0xCBE6] = instruction::set_ar16(4, Registers16::HL);
    vec[0xCBE7] = instruction::set_r8(4, Registers8::A);

    vec[0xCBE8] = instruction::set_r8(5, Registers8::B);
    vec[0xCBE9] = instruction::set_r8(5, Registers8::C);
    vec[0xCBEA] = instruction::set_r8(5, Registers8::D);
    vec[0xCBEB] = instruction::set_r8(5, Registers8::E);
    vec[0xCBEC] = instruction::set_r8(5, Registers8::H);
    vec[0xCBED] = instruction::set_r8(5, Registers8::L);
    vec[0xCBEE] = instruction::set_ar16(5, Registers16::HL);
    vec[0xCBEF] = instruction::set_r8(5, Registers8::A);

    vec[0xCBF0] = instruction::set_r8(6, Registers8::B);
    vec[0xCBF1] = instruction::set_r8(6, Registers8::C);
    vec[0xCBF2] = instruction::set_r8(6, Registers8::D);
    vec[0xCBF3] = instruction::set_r8(6, Registers8::E);
    vec[0xCBF4] = instruction::set_r8(6, Registers8::H);
    vec[0xCBF5] = instruction::set_r8(6, Registers8::L);
    vec[0xCBF6] = instruction::set_ar16(6, Registers16::HL);
    vec[0xCBF7] = instruction::set_r8(6, Registers8::A);

    vec[0xCBF8] = instruction::set_r8(7, Registers8::B);
    vec[0xCBF9] = instruction::set_r8(7, Registers8::C);
    vec[0xCBFA] = instruction::set_r8(7, Registers8::D);
    vec[0xCBFB] = instruction::set_r8(7, Registers8::E);
    vec[0xCBFC] = instruction::set_r8(7, Registers8::H);
    vec[0xCBFD] = instruction::set_r8(7, Registers8::L);
    vec[0xCBFE] = instruction::set_ar16(7, Registers16::HL);
    vec[0xCBFF] = instruction::set_r8(7, Registers8::A);

    vec
}
