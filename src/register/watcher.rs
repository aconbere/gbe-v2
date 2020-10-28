use std::collections::HashSet;

use super::{Registers8, Registers16};

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
        }
    }

    pub fn insert8(&mut self, r: Registers8, v: u8) -> bool {
        match r {
            Registers8::A => self.a.insert(v),
            Registers8::B => self.b.insert(v),
            Registers8::C => self.c.insert(v),
            Registers8::D => self.d.insert(v),
            Registers8::E => self.e.insert(v),
            Registers8::F => self.f.insert(v),
            Registers8::H => self.h.insert(v),
            Registers8::L => self.l.insert(v),
        }
    }

    pub fn insert16(&mut self, r: Registers16, v: u16) -> bool {
        match r {
            Registers16::AF => self.af.insert(v),
            Registers16::BC => self.bc.insert(v),
            Registers16::DE => self.de.insert(v),
            Registers16::HL => self.hl.insert(v),
            Registers16::PC => self.pc.insert(v),
            Registers16::SP => self.sp.insert(v),
        }
    }

    pub fn contains8(&mut self, r: Registers8, v: u8) -> bool {
        match r {
            Registers8::A => self.a.contains(&v),
            Registers8::B => self.b.contains(&v),
            Registers8::C => self.c.contains(&v),
            Registers8::D => self.d.contains(&v),
            Registers8::E => self.e.contains(&v),
            Registers8::F => self.f.contains(&v),
            Registers8::H => self.h.contains(&v),
            Registers8::L => self.l.contains(&v),
        }
    }

    pub fn contains16(&mut self, r: Registers16, v: u16) -> bool {
        match r {
            Registers16::AF => self.af.contains(&v),
            Registers16::BC => self.bc.contains(&v),
            Registers16::DE => self.de.contains(&v),
            Registers16::HL => self.hl.contains(&v),
            Registers16::PC => self.pc.contains(&v),
            Registers16::SP => self.sp.contains(&v),
        }
    }

    pub fn remove8(&mut self, r: Registers8, v: u8) -> bool {
        match r {
            Registers8::A => self.a.remove(&v),
            Registers8::B => self.b.remove(&v),
            Registers8::C => self.c.remove(&v),
            Registers8::D => self.d.remove(&v),
            Registers8::E => self.e.remove(&v),
            Registers8::F => self.f.remove(&v),
            Registers8::H => self.h.remove(&v),
            Registers8::L => self.l.remove(&v),
        }
    }

    pub fn remove16(&mut self, r: Registers16, v: u16) -> bool {
        match r {
            Registers16::AF => self.af.remove(&v),
            Registers16::BC => self.bc.remove(&v),
            Registers16::DE => self.de.remove(&v),
            Registers16::HL => self.hl.remove(&v),
            Registers16::PC => self.pc.remove(&v),
            Registers16::SP => self.sp.remove(&v),
        }
    }
}
