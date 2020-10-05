// use crate::register::{Registers8, Registers16};
// use std::collections::HashSet;
// 
// pub enum Target {
//     R8(Registers8, u8),
//     R16(Registers16, u16),
//     Mem((u16, u8)),
// }
// 
// pub struct WatchList {
//     a: HashSet<u8>,
//     b: HashSet<u8>,
//     c: HashSet<u8>,
//     d: HashSet<u8>,
//     e: HashSet<u8>,
//     f: HashSet<u8>,
//     h: HashSet<u8>,
//     l: HashSet<u8>,
//     af: HashSet<u16>,
//     bc: HashSet<u16>,
//     de: HashSet<u16>,
//     hl: HashSet<u16>,
//     pc: HashSet<u16>,
//     sp: HashSet<u16>,
//     mem: HashSet<(u16, u8)>,
// }
// 
// 
// impl WatchList {
//     pub fn get8(&self, t: Registers8) -> HashSet<u8> {
//         match t {
//             Registers8::A => self.a,
//             Registers8::B => self.b,
//             Registers8::C => self.c,
//             Registers8::D => self.d,
//             Registers8::E => self.e,
//             Registers8::F => self.f,
//             Registers8::H => self.h,
//             Registers8::L => self.l,
//         }
//     }
// 
//     pub fn get16(&self, t: Registers16) -> HashSet<u16> {
//         match t {
//             Registers16::AF => self.af,
//             Registers16::BC => self.bc,
//             Registers16::DE => self.de,
//             Registers16::HL => self.hl,
//             Registers16::SP => self.sp,
//             Registers16::PC => self.pc,
//         }
//     }
//         
//     pub fn add8(&mut self, t: Registers8, value: u8) -> bool {
//         self.get8(t).insert(value)
//     }
// 
//     pub fn add16(&mut self, t: Registers16, value: u16) -> bool {
//         self.get16(t).insert(value)
//     }
// 
//     pub fn add_mem(&mut self, address: u16, value: u8) -> bool {
//         self.mem.insert((address, value))
//     }
// 
//     pub fn delete8(&mut self, t: Registers8, value: &u8) -> bool {
//         self.get8(t).remove(value)
//     }
// 
//     pub fn delete16(&mut self, t: Registers16, value: &u16) -> bool {
//         self.get16(t).remove(value)
//     }
// 
//     pub fn delete_mem(&mut self, address: u16, value: u8) -> bool {
//         self.mem.remove(&(address, value))
//     }
// 
//     pub fn check(&self, target: Target) -> bool {
//         match target {
//             Target::R8(t, v) => self.get8(t).contains(&v),
//             Target::R16(t, v) => self.get16(t).contains(&v),
//             Target::Mem(t) => self.mem.contains(&t),
//         }
//     }
// 
// }
