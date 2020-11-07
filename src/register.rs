use super::bytes;
use std::fmt;
use std::fmt::Debug;

pub mod watcher;

use watcher::Watcher;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum IME {
    Enabled,
    Disabled,
    Queued,
}

impl IME {
    pub fn queued(&self) -> bool {
        self == &IME::Queued
    }

    pub fn enabled(&self) -> bool {
        self == &IME::Enabled
    }

    pub fn flagged_on(&self) -> bool {
        self == &IME::Queued || self == &IME::Enabled
    }
}

pub enum R {
    R8(Registers8),
    R16(Registers16),
}

pub enum RValue {
    R8(u8),
    R16(u16),
}

impl RValue {
    pub fn get8(&self) -> u8 {
        match &self {
            RValue::R8(v) => *v,
            RValue::R16(_) => panic!("can't coerce to u16 from an R8"),
        }
    }

    pub fn get16(&self) -> u16 {
        match &self {
            RValue::R8(_) => panic!("can't coerce to u8 from an R16"),
            RValue::R16(v) => *v,
        }
    }
}

pub enum RPair {
    R8(Registers8, u8),
    R16(Registers16, u16),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub stopped: bool,
    pub watcher: Watcher,
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
            stopped: false,
            watcher: Watcher::new(),
        };
    }

    pub fn skip_boot() -> Registers {
        return Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
            ime: IME::Disabled,
            stopped: false,
            watcher: Watcher::new(),
        };
    }

    pub fn get(&self, r: R) -> RValue {
        match r {
            R::R8(Registers8::A) => RValue::R8(self.a),
            R::R8(Registers8::B) => RValue::R8(self.b),
            R::R8(Registers8::C) => RValue::R8(self.c),
            R::R8(Registers8::D) => RValue::R8(self.d),
            R::R8(Registers8::E) => RValue::R8(self.e),
            R::R8(Registers8::F) => RValue::R8(self.f),
            R::R8(Registers8::H) => RValue::R8(self.h),
            R::R8(Registers8::L) => RValue::R8(self.l),

            R::R16(Registers16::AF) => RValue::R16(bytes::combine_ms_ls(self.a, self.f)),
            R::R16(Registers16::BC) => RValue::R16(bytes::combine_ms_ls(self.b, self.c)),
            R::R16(Registers16::DE) => RValue::R16(bytes::combine_ms_ls(self.d, self.e)),
            R::R16(Registers16::HL) => RValue::R16(bytes::combine_ms_ls(self.h, self.l)),
            R::R16(Registers16::PC) => RValue::R16(self.pc),
            R::R16(Registers16::SP) => RValue::R16(self.sp),
        }
    }

    pub fn get8(&self, r: Registers8) -> u8 {
        self.get(R::R8(r)).get8()
    }

    pub fn get16(&self, r: Registers16) -> u16 {
        self.get(R::R16(r)).get16()
    }

    pub fn set(&mut self, r: RPair) {
        match r {
            RPair::R8(Registers8::A, v) => self.a = v,
            RPair::R8(Registers8::B, v) => self.b = v,
            RPair::R8(Registers8::C, v) => self.c = v,
            RPair::R8(Registers8::D, v) => self.d = v,
            RPair::R8(Registers8::E, v) => self.e = v,
            // least significant nibble in f is always zero
            RPair::R8(Registers8::F, v) => self.f = v & 0xF0,
            RPair::R8(Registers8::H, v) => self.h = v,
            RPair::R8(Registers8::L, v) => self.l = v,

            RPair::R16(Registers16::AF, v) => self.set_combined(Registers8::A, Registers8::F, v),
            RPair::R16(Registers16::BC, v) => self.set_combined(Registers8::B, Registers8::C, v),
            RPair::R16(Registers16::DE, v) => self.set_combined(Registers8::D, Registers8::E, v),
            RPair::R16(Registers16::HL, v) => self.set_combined(Registers8::H, Registers8::L, v),
            RPair::R16(Registers16::SP, v) => self.sp = v,
            RPair::R16(Registers16::PC, v) => self.pc = v,
        }

        self.watcher.check(r);
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
        self.set(RPair::R16(r, v));
    }

    fn set_combined(&mut self, r1: Registers8, r2: Registers8, v: u16) {
        let (ms, ls) = bytes::split_ms_ls(v);
        self.set(RPair::R8(r1, ms));
        self.set(RPair::R8(r2, ls));
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
