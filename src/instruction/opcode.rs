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
    vec.resize_with(512, || { instruction::nop() });

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

    vec[0x0100] = instruction::rlc_r8(Registers8::B);
    vec[0x0101] = instruction::rlc_r8(Registers8::C);
    vec[0x0102] = instruction::rlc_r8(Registers8::D);
    vec[0x0103] = instruction::rlc_r8(Registers8::E);
    vec[0x0104] = instruction::rlc_r8(Registers8::H);
    vec[0x0105] = instruction::rlc_r8(Registers8::L);
    vec[0x0106] = instruction::rlc_ar16(Registers16::HL);
    vec[0x0107] = instruction::rlc_r8(Registers8::A);

    vec[0x0108] = instruction::rrc_r8(Registers8::B);
    vec[0x0109] = instruction::rrc_r8(Registers8::C);
    vec[0x010A] = instruction::rrc_r8(Registers8::D);
    vec[0x010B] = instruction::rrc_r8(Registers8::E);
    vec[0x010C] = instruction::rrc_r8(Registers8::H);
    vec[0x010D] = instruction::rrc_r8(Registers8::L);
    vec[0x010E] = instruction::rrc_ar16(Registers16::HL);
    vec[0x010F] = instruction::rrc_r8(Registers8::A);

    vec[0x0110] = instruction::rl_r8(Registers8::B);
    vec[0x0111] = instruction::rl_r8(Registers8::C);
    vec[0x0112] = instruction::rl_r8(Registers8::D);
    vec[0x0113] = instruction::rl_r8(Registers8::E);
    vec[0x0114] = instruction::rl_r8(Registers8::H);
    vec[0x0115] = instruction::rl_r8(Registers8::L);
    vec[0x0116] = instruction::rl_ar16(Registers16::HL);
    vec[0x0117] = instruction::rl_r8(Registers8::A);

    vec[0x0118] = instruction::rr_r8(Registers8::B);
    vec[0x0119] = instruction::rr_r8(Registers8::C);
    vec[0x011A] = instruction::rr_r8(Registers8::D);
    vec[0x011B] = instruction::rr_r8(Registers8::E);
    vec[0x011C] = instruction::rr_r8(Registers8::H);
    vec[0x011D] = instruction::rr_r8(Registers8::L);
    vec[0x011E] = instruction::rr_ar16(Registers16::HL);
    vec[0x011F] = instruction::rr_r8(Registers8::A);

    vec[0x0120] = instruction::sla_r8(Registers8::B);
    vec[0x0121] = instruction::sla_r8(Registers8::C);
    vec[0x0122] = instruction::sla_r8(Registers8::D);
    vec[0x0123] = instruction::sla_r8(Registers8::E);
    vec[0x0124] = instruction::sla_r8(Registers8::H);
    vec[0x0125] = instruction::sla_r8(Registers8::L);
    vec[0x0126] = instruction::sla_ar16(Registers16::HL);
    vec[0x0127] = instruction::sla_r8(Registers8::A);

    vec[0x0128] = instruction::sra_r8(Registers8::B);
    vec[0x0129] = instruction::sra_r8(Registers8::C);
    vec[0x012A] = instruction::sra_r8(Registers8::D);
    vec[0x012B] = instruction::sra_r8(Registers8::E);
    vec[0x012C] = instruction::sra_r8(Registers8::H);
    vec[0x012D] = instruction::sra_r8(Registers8::L);
    vec[0x012E] = instruction::sra_ar16(Registers16::HL);
    vec[0x012F] = instruction::sra_r8(Registers8::A);

    vec[0x0130] = instruction::swap_r8(Registers8::B);
    vec[0x0131] = instruction::swap_r8(Registers8::C);
    vec[0x0132] = instruction::swap_r8(Registers8::D);
    vec[0x0133] = instruction::swap_r8(Registers8::E);
    vec[0x0134] = instruction::swap_r8(Registers8::H);
    vec[0x0135] = instruction::swap_r8(Registers8::L);
    vec[0x0136] = instruction::swap_ar16(Registers16::HL);
    vec[0x0137] = instruction::swap_r8(Registers8::A);

    vec[0x0138] = instruction::srl_r8(Registers8::B);
    vec[0x0139] = instruction::srl_r8(Registers8::C);
    vec[0x013A] = instruction::srl_r8(Registers8::D);
    vec[0x013B] = instruction::srl_r8(Registers8::E);
    vec[0x013C] = instruction::srl_r8(Registers8::H);
    vec[0x013D] = instruction::srl_r8(Registers8::L);
    vec[0x013E] = instruction::srl_ar16(Registers16::HL);
    vec[0x013F] = instruction::srl_r8(Registers8::A);

    vec[0x0140] = instruction::bit_r8(0, Registers8::B);
    vec[0x0141] = instruction::bit_r8(0, Registers8::C);
    vec[0x0142] = instruction::bit_r8(0, Registers8::D);
    vec[0x0143] = instruction::bit_r8(0, Registers8::E);
    vec[0x0144] = instruction::bit_r8(0, Registers8::H);
    vec[0x0145] = instruction::bit_r8(0, Registers8::L);
    vec[0x0146] = instruction::bit_ar16(0, Registers16::HL);
    vec[0x0147] = instruction::bit_r8(0, Registers8::A);

    vec[0x0148] = instruction::bit_r8(1, Registers8::B);
    vec[0x0149] = instruction::bit_r8(1, Registers8::C);
    vec[0x014A] = instruction::bit_r8(1, Registers8::D);
    vec[0x014B] = instruction::bit_r8(1, Registers8::E);
    vec[0x014C] = instruction::bit_r8(1, Registers8::H);
    vec[0x014D] = instruction::bit_r8(1, Registers8::L);
    vec[0x014E] = instruction::bit_ar16(1, Registers16::HL);
    vec[0x014F] = instruction::bit_r8(1, Registers8::A);

    vec[0x0150] = instruction::bit_r8(2, Registers8::B);
    vec[0x0151] = instruction::bit_r8(2, Registers8::C);
    vec[0x0152] = instruction::bit_r8(2, Registers8::D);
    vec[0x0153] = instruction::bit_r8(2, Registers8::E);
    vec[0x0154] = instruction::bit_r8(2, Registers8::H);
    vec[0x0155] = instruction::bit_r8(2, Registers8::L);
    vec[0x0156] = instruction::bit_ar16(2, Registers16::HL);
    vec[0x0157] = instruction::bit_r8(2, Registers8::A);

    vec[0x0158] = instruction::bit_r8(3, Registers8::B);
    vec[0x0159] = instruction::bit_r8(3, Registers8::C);
    vec[0x015A] = instruction::bit_r8(3, Registers8::D);
    vec[0x015B] = instruction::bit_r8(3, Registers8::E);
    vec[0x015C] = instruction::bit_r8(3, Registers8::H);
    vec[0x015D] = instruction::bit_r8(3, Registers8::L);
    vec[0x015E] = instruction::bit_ar16(3, Registers16::HL);
    vec[0x015F] = instruction::bit_r8(3, Registers8::A);

    vec[0x0160] = instruction::bit_r8(4, Registers8::B);
    vec[0x0161] = instruction::bit_r8(4, Registers8::C);
    vec[0x0162] = instruction::bit_r8(4, Registers8::D);
    vec[0x0163] = instruction::bit_r8(4, Registers8::E);
    vec[0x0164] = instruction::bit_r8(4, Registers8::H);
    vec[0x0165] = instruction::bit_r8(4, Registers8::L);
    vec[0x0166] = instruction::bit_ar16(4, Registers16::HL);
    vec[0x0167] = instruction::bit_r8(4, Registers8::A);

    vec[0x0168] = instruction::bit_r8(5, Registers8::B);
    vec[0x0169] = instruction::bit_r8(5, Registers8::C);
    vec[0x016A] = instruction::bit_r8(5, Registers8::D);
    vec[0x016B] = instruction::bit_r8(5, Registers8::E);
    vec[0x016C] = instruction::bit_r8(5, Registers8::H);
    vec[0x016D] = instruction::bit_r8(5, Registers8::L);
    vec[0x016E] = instruction::bit_ar16(5, Registers16::HL);
    vec[0x016F] = instruction::bit_r8(5, Registers8::A);

    vec[0x0170] = instruction::bit_r8(6, Registers8::B);
    vec[0x0171] = instruction::bit_r8(6, Registers8::C);
    vec[0x0172] = instruction::bit_r8(6, Registers8::D);
    vec[0x0173] = instruction::bit_r8(6, Registers8::E);
    vec[0x0174] = instruction::bit_r8(6, Registers8::H);
    vec[0x0175] = instruction::bit_r8(6, Registers8::L);
    vec[0x0176] = instruction::bit_ar16(6, Registers16::HL);
    vec[0x0177] = instruction::bit_r8(6, Registers8::A);

    vec[0x0178] = instruction::bit_r8(7, Registers8::B);
    vec[0x0179] = instruction::bit_r8(7, Registers8::C);
    vec[0x017A] = instruction::bit_r8(7, Registers8::D);
    vec[0x017B] = instruction::bit_r8(7, Registers8::E);
    vec[0x017C] = instruction::bit_r8(7, Registers8::H);
    vec[0x017D] = instruction::bit_r8(7, Registers8::L);
    vec[0x017E] = instruction::bit_ar16(7, Registers16::HL);
    vec[0x017F] = instruction::bit_r8(7, Registers8::A);

    vec[0x0180] = instruction::res_r8(0, Registers8::B);
    vec[0x0181] = instruction::res_r8(0, Registers8::C);
    vec[0x0182] = instruction::res_r8(0, Registers8::D);
    vec[0x0183] = instruction::res_r8(0, Registers8::E);
    vec[0x0184] = instruction::res_r8(0, Registers8::H);
    vec[0x0185] = instruction::res_r8(0, Registers8::L);
    vec[0x0186] = instruction::res_ar16(0, Registers16::HL);
    vec[0x0187] = instruction::res_r8(0, Registers8::A);

    vec[0x0188] = instruction::res_r8(1, Registers8::B);
    vec[0x0189] = instruction::res_r8(1, Registers8::C);
    vec[0x018A] = instruction::res_r8(1, Registers8::D);
    vec[0x018B] = instruction::res_r8(1, Registers8::E);
    vec[0x018C] = instruction::res_r8(1, Registers8::H);
    vec[0x018D] = instruction::res_r8(1, Registers8::L);
    vec[0x018E] = instruction::res_ar16(1, Registers16::HL);
    vec[0x018F] = instruction::res_r8(1, Registers8::A);

    vec[0x0190] = instruction::res_r8(2, Registers8::B);
    vec[0x0191] = instruction::res_r8(2, Registers8::C);
    vec[0x0192] = instruction::res_r8(2, Registers8::D);
    vec[0x0193] = instruction::res_r8(2, Registers8::E);
    vec[0x0194] = instruction::res_r8(2, Registers8::H);
    vec[0x0195] = instruction::res_r8(2, Registers8::L);
    vec[0x0196] = instruction::res_ar16(2, Registers16::HL);
    vec[0x0197] = instruction::res_r8(2, Registers8::A);

    vec[0x0198] = instruction::res_r8(3, Registers8::B);
    vec[0x0199] = instruction::res_r8(3, Registers8::C);
    vec[0x019A] = instruction::res_r8(3, Registers8::D);
    vec[0x019B] = instruction::res_r8(3, Registers8::E);
    vec[0x019C] = instruction::res_r8(3, Registers8::H);
    vec[0x019D] = instruction::res_r8(3, Registers8::L);
    vec[0x019E] = instruction::res_ar16(3, Registers16::HL);
    vec[0x019F] = instruction::res_r8(3, Registers8::A);

    vec[0x01A0] = instruction::res_r8(4, Registers8::B);
    vec[0x01A1] = instruction::res_r8(4, Registers8::C);
    vec[0x01A2] = instruction::res_r8(4, Registers8::D);
    vec[0x01A3] = instruction::res_r8(4, Registers8::E);
    vec[0x01A4] = instruction::res_r8(4, Registers8::H);
    vec[0x01A5] = instruction::res_r8(4, Registers8::L);
    vec[0x01A6] = instruction::res_ar16(4, Registers16::HL);
    vec[0x01A7] = instruction::res_r8(4, Registers8::A);

    vec[0x01A8] = instruction::res_r8(5, Registers8::B);
    vec[0x01A9] = instruction::res_r8(5, Registers8::C);
    vec[0x01AA] = instruction::res_r8(5, Registers8::D);
    vec[0x01AB] = instruction::res_r8(5, Registers8::E);
    vec[0x01AC] = instruction::res_r8(5, Registers8::H);
    vec[0x01AD] = instruction::res_r8(5, Registers8::L);
    vec[0x01AE] = instruction::res_ar16(5, Registers16::HL);
    vec[0x01AF] = instruction::res_r8(5, Registers8::A);

    vec[0x01B0] = instruction::res_r8(6, Registers8::B);
    vec[0x01B1] = instruction::res_r8(6, Registers8::C);
    vec[0x01B2] = instruction::res_r8(6, Registers8::D);
    vec[0x01B3] = instruction::res_r8(6, Registers8::E);
    vec[0x01B4] = instruction::res_r8(6, Registers8::H);
    vec[0x01B5] = instruction::res_r8(6, Registers8::L);
    vec[0x01B6] = instruction::res_ar16(6, Registers16::HL);
    vec[0x01B7] = instruction::res_r8(6, Registers8::A);

    vec[0x01B8] = instruction::res_r8(7, Registers8::B);
    vec[0x01B9] = instruction::res_r8(7, Registers8::C);
    vec[0x01BA] = instruction::res_r8(7, Registers8::D);
    vec[0x01BB] = instruction::res_r8(7, Registers8::E);
    vec[0x01BC] = instruction::res_r8(7, Registers8::H);
    vec[0x01BD] = instruction::res_r8(7, Registers8::L);
    vec[0x01BE] = instruction::res_ar16(7, Registers16::HL);
    vec[0x01BF] = instruction::res_r8(7, Registers8::A);

    vec[0x01C0] = instruction::set_r8(0, Registers8::B);
    vec[0x01C1] = instruction::set_r8(0, Registers8::C);
    vec[0x01C2] = instruction::set_r8(0, Registers8::D);
    vec[0x01C3] = instruction::set_r8(0, Registers8::E);
    vec[0x01C4] = instruction::set_r8(0, Registers8::H);
    vec[0x01C5] = instruction::set_r8(0, Registers8::L);
    vec[0x01C6] = instruction::set_ar16(0, Registers16::HL);
    vec[0x01C7] = instruction::set_r8(0, Registers8::A);

    vec[0x01C8] = instruction::set_r8(1, Registers8::B);
    vec[0x01C9] = instruction::set_r8(1, Registers8::C);
    vec[0x01CA] = instruction::set_r8(1, Registers8::D);
    vec[0x01CB] = instruction::set_r8(1, Registers8::E);
    vec[0x01CC] = instruction::set_r8(1, Registers8::H);
    vec[0x01CD] = instruction::set_r8(1, Registers8::L);
    vec[0x01CE] = instruction::set_ar16(1, Registers16::HL);
    vec[0x01CF] = instruction::set_r8(1, Registers8::A);

    vec[0x01D0] = instruction::set_r8(2, Registers8::B);
    vec[0x01D1] = instruction::set_r8(2, Registers8::C);
    vec[0x01D2] = instruction::set_r8(2, Registers8::D);
    vec[0x01D3] = instruction::set_r8(2, Registers8::E);
    vec[0x01D4] = instruction::set_r8(2, Registers8::H);
    vec[0x01D5] = instruction::set_r8(2, Registers8::L);
    vec[0x01D6] = instruction::set_ar16(2, Registers16::HL);
    vec[0x01D7] = instruction::set_r8(2, Registers8::A);

    vec[0x01D8] = instruction::set_r8(3, Registers8::B);
    vec[0x01D9] = instruction::set_r8(3, Registers8::C);
    vec[0x01DA] = instruction::set_r8(3, Registers8::D);
    vec[0x01DB] = instruction::set_r8(3, Registers8::E);
    vec[0x01DC] = instruction::set_r8(3, Registers8::H);
    vec[0x01DD] = instruction::set_r8(3, Registers8::L);
    vec[0x01DE] = instruction::set_ar16(3, Registers16::HL);
    vec[0x01DF] = instruction::set_r8(3, Registers8::A);

    vec[0x01E0] = instruction::set_r8(4, Registers8::B);
    vec[0x01E1] = instruction::set_r8(4, Registers8::C);
    vec[0x01E2] = instruction::set_r8(4, Registers8::D);
    vec[0x01E3] = instruction::set_r8(4, Registers8::E);
    vec[0x01E4] = instruction::set_r8(4, Registers8::H);
    vec[0x01E5] = instruction::set_r8(4, Registers8::L);
    vec[0x01E6] = instruction::set_ar16(4, Registers16::HL);
    vec[0x01E7] = instruction::set_r8(4, Registers8::A);

    vec[0x01E8] = instruction::set_r8(5, Registers8::B);
    vec[0x01E9] = instruction::set_r8(5, Registers8::C);
    vec[0x01EA] = instruction::set_r8(5, Registers8::D);
    vec[0x01EB] = instruction::set_r8(5, Registers8::E);
    vec[0x01EC] = instruction::set_r8(5, Registers8::H);
    vec[0x01ED] = instruction::set_r8(5, Registers8::L);
    vec[0x01EE] = instruction::set_ar16(5, Registers16::HL);
    vec[0x01EF] = instruction::set_r8(5, Registers8::A);

    vec[0x01F0] = instruction::set_r8(6, Registers8::B);
    vec[0x01F1] = instruction::set_r8(6, Registers8::C);
    vec[0x01F2] = instruction::set_r8(6, Registers8::D);
    vec[0x01F3] = instruction::set_r8(6, Registers8::E);
    vec[0x01F4] = instruction::set_r8(6, Registers8::H);
    vec[0x01F5] = instruction::set_r8(6, Registers8::L);
    vec[0x01F6] = instruction::set_ar16(6, Registers16::HL);
    vec[0x01F7] = instruction::set_r8(6, Registers8::A);

    vec[0x01F8] = instruction::set_r8(7, Registers8::B);
    vec[0x01F9] = instruction::set_r8(7, Registers8::C);
    vec[0x01FA] = instruction::set_r8(7, Registers8::D);
    vec[0x01FB] = instruction::set_r8(7, Registers8::E);
    vec[0x01FC] = instruction::set_r8(7, Registers8::H);
    vec[0x01FD] = instruction::set_r8(7, Registers8::L);
    vec[0x01FE] = instruction::set_ar16(7, Registers16::HL);
    vec[0x01FF] = instruction::set_r8(7, Registers8::A);

    vec
}
