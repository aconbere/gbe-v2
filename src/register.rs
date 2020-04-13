use super::bytes;
use std::fmt;
use std::fmt::Debug;

pub enum IME {
    Enabled,
    Disabled,
    Queued,
}

#[derive(Debug, Clone, Copy)]
pub enum Registers8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Registers16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

pub enum Flag {
    Z,
    C,
    N,
    H,
}

impl Flag {
    pub fn get_index(&self) -> u8 {
        match self {
            Flag::Z => 7,
            Flag::N => 6,
            Flag::H => 5,
            Flag::C => 4,
        }
    }
}


/* 16 bit combined registers
 *
 * Taking the case of HL for example, it is the combination of the 8 bit registers H and L
 * Data stored in HL are stored in little endian order, that is the most significant byte
 * is stored Last.
 *
 * so if we wanted to store 1024 (0400 in hex) then we want to store into HL
 *
 * H = 00
 * L = 04
 *
 * and when we read it out we want to do
 *
 * L << 8 | H
 */

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    pub ime: IME,
}

impl Registers {
    pub fn new() -> Registers {
        return Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE,
            pc: 0x0000,
            ime: IME::Disabled,
        };
    }

    pub fn get8(&self, r: Registers8) -> u8 {
        match r {
            Registers8::A => self.a,
            Registers8::B => self.b,
            Registers8::C => self.c,
            Registers8::D => self.d,
            Registers8::E => self.e,
            Registers8::F => self.f,
            Registers8::H => self.h,
            Registers8::L => self.l,
        }
    }
    pub fn get16(&self, r: Registers16) -> u16 {
        match r {
            Registers16::AF => bytes::combine_ms_ls(self.a, self.f),
            Registers16::BC => bytes::combine_ms_ls(self.b, self.c),
            Registers16::DE => bytes::combine_ms_ls(self.d, self.e),
            Registers16::HL => bytes::combine_ms_ls(self.h, self.l),
            Registers16::PC => self.pc,
            Registers16::SP => self.sp,
        }
    }

    pub fn set8(&mut self, r: Registers8, v: u8) {
        match r {
            Registers8::A => self.a = v,
            Registers8::B => self.b = v,
            Registers8::C => self.c = v,
            Registers8::D => self.d = v,
            Registers8::E => self.e = v,
            // least significant nibble in f is always zero
            Registers8::F => self.f = v & 0xF0,
            Registers8::H => self.h = v,
            Registers8::L => self.l = v,
        }
    }

    pub fn set16(&mut self, r: Registers16, v: u16) {
        match r {
            Registers16::AF => self.set_combined(Registers8::A, Registers8::F, v),
            Registers16::BC => self.set_combined(Registers8::B, Registers8::C, v),
            Registers16::DE => self.set_combined(Registers8::D, Registers8::E, v),
            Registers16::HL => self.set_combined(Registers8::H, Registers8::L, v),
            Registers16::PC => self.pc = v,
            Registers16::SP => self.sp = v,
        }
    }

    pub fn set_flag(&mut self, f: Flag, check: bool) {
        self.f = bytes::set_bit(self.f, f.get_index(), check);
    }

    pub fn get_flag(&self, f: Flag) -> bool {
        bytes::check_bit(self.f, f.get_index())
    }

    pub fn inc16(&mut self, r:Registers16) {
        let v = self.get16(r).wrapping_add(1);
        self.set16(r, v);
    }

    fn set_combined(&mut self, r1: Registers8, r2: Registers8, v: u16) {
        let (ms, ls) = bytes::split_ms_ls(v);
        self.set8(r1, ms);
        self.set8(r2, ls);
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "AF: {:04X} BC: {:04X} DE: {:04X} HL: {:04X} PC: {:04X} SP {:04X}",
               self.get16(Registers16::AF),
               self.get16(Registers16::BC),
               self.get16(Registers16::DE),
               self.get16(Registers16::HL),
               self.get16(Registers16::PC),
               self.get16(Registers16::SP),
        )
    } 

}
