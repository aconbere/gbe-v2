use std::collections::HashSet;

use super::{Registers8, Registers16, RPair};

pub struct Watcher {
    pub a: HashSet<u8>,
    pub b: HashSet<u8>,
    pub c: HashSet<u8>,
    pub d: HashSet<u8>,
    pub e: HashSet<u8>,
    pub f: HashSet<u8>,
    pub h: HashSet<u8>,
    pub l: HashSet<u8>,
    pub af: HashSet<u16>,
    pub bc: HashSet<u16>,
    pub de: HashSet<u16>,
    pub hl: HashSet<u16>,
    pub sp: HashSet<u16>,
    pub pc: HashSet<u16>,

    pub breaks: Vec<RPair>
}

impl Watcher {
    pub fn new() -> Watcher {
        Watcher {
            a: HashSet::new(),
            b: HashSet::new(),
            c: HashSet::new(),
            d: HashSet::new(),
            e: HashSet::new(),
            f: HashSet::new(),
            h: HashSet::new(),
            l: HashSet::new(),
            af: HashSet::new(),
            bc: HashSet::new(),
            de: HashSet::new(),
            hl: HashSet::new(),
            sp: HashSet::new(),
            pc: HashSet::new(),

            breaks: Vec::new(),
        }
    }

    pub fn add_break_point(&mut self, r: RPair) -> bool {
        match r {
            RPair::R8(Registers8::A, v) => self.a.insert(v),
            RPair::R8(Registers8::B, v) => self.b.insert(v),
            RPair::R8(Registers8::C, v) => self.c.insert(v),
            RPair::R8(Registers8::D, v) => self.d.insert(v),
            RPair::R8(Registers8::E, v) => self.e.insert(v),
            RPair::R8(Registers8::F, v) => self.f.insert(v),
            RPair::R8(Registers8::H, v) => self.h.insert(v),
            RPair::R8(Registers8::L, v) => self.l.insert(v),
            RPair::R16(Registers16::AF, v) => self.af.insert(v),
            RPair::R16(Registers16::BC, v) => self.bc.insert(v),
            RPair::R16(Registers16::DE, v) => self.de.insert(v),
            RPair::R16(Registers16::HL, v) => self.hl.insert(v),
            RPair::R16(Registers16::PC, v) => self.pc.insert(v),
            RPair::R16(Registers16::SP, v) => self.sp.insert(v),
        }
    }

    pub fn check(&mut self, r: RPair) -> bool {
        let contains = match r {
            RPair::R8(Registers8::A, v) => self.a.contains(&v),
            RPair::R8(Registers8::B, v) => self.b.contains(&v),
            RPair::R8(Registers8::C, v) => self.c.contains(&v),
            RPair::R8(Registers8::D, v) => self.d.contains(&v),
            RPair::R8(Registers8::E, v) => self.e.contains(&v),
            RPair::R8(Registers8::F, v) => self.f.contains(&v),
            RPair::R8(Registers8::H, v) => self.h.contains(&v),
            RPair::R8(Registers8::L, v) => self.l.contains(&v),
            RPair::R16(Registers16::AF, v) => self.af.contains(&v),
            RPair::R16(Registers16::BC, v) => self.bc.contains(&v),
            RPair::R16(Registers16::DE, v) => self.de.contains(&v),
            RPair::R16(Registers16::HL, v) => self.hl.contains(&v),
            RPair::R16(Registers16::PC, v) => self.pc.contains(&v),
            RPair::R16(Registers16::SP, v) => self.sp.contains(&v),
        };

        if contains {
            self.breaks.push(r);
        }

        contains
    }

    pub fn remove_break_point(&mut self, r: RPair) -> bool {
        match r {
            RPair::R8(Registers8::A, v) => self.a.remove(&v),
            RPair::R8(Registers8::B, v) => self.b.remove(&v),
            RPair::R8(Registers8::C, v) => self.c.remove(&v),
            RPair::R8(Registers8::D, v) => self.d.remove(&v),
            RPair::R8(Registers8::E, v) => self.e.remove(&v),
            RPair::R8(Registers8::F, v) => self.f.remove(&v),
            RPair::R8(Registers8::H, v) => self.h.remove(&v),
            RPair::R8(Registers8::L, v) => self.l.remove(&v),
            RPair::R16(Registers16::AF, v) => self.af.remove(&v),
            RPair::R16(Registers16::BC, v) => self.bc.remove(&v),
            RPair::R16(Registers16::DE, v) => self.de.remove(&v),
            RPair::R16(Registers16::HL, v) => self.hl.remove(&v),
            RPair::R16(Registers16::PC, v) => self.pc.remove(&v),
            RPair::R16(Registers16::SP, v) => self.sp.remove(&v),
        }
    }
}
