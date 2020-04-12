use crate::register::{Registers16, Registers8, Flag};

use crate::bytes;
use crate::cpu::CPU;

#[derive(Debug, Clone, Copy)]
pub enum RstFlag {
    H00, H08, H10, H18, H20, H28, H30, H38
}

fn rst_locations(f: RstFlag) -> u8 {
    match f {
        RstFlag::H00 => 0x00,
        RstFlag::H08 => 0x08,
        RstFlag::H10 => 0x10,
        RstFlag::H18 => 0x18,
        RstFlag::H20 => 0x20,
        RstFlag::H28 => 0x28,
        RstFlag::H30 => 0x30,
        RstFlag::H38 => 0x38,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JumpFlag {
    NZ, Z, NC, C
}

/* Helper Functions */

fn _set(location: u8, v: u8) -> u8 {
    v | (1 << location)
}

/* Resets to 0 the specified bit in the specified register r
 */
fn _res(location: u8, v: u8) -> u8 {
    v & !(1 << location)
}

/* Copies the complement of the contents of the specified bit in register r to the Z flag of the
 * program status word (PSW).
*/
fn _bit(cpu: &mut CPU, location:u8, v:u8) {
    let out = bytes::check_bit(v, location);

    cpu.registers.set_flag(Flag::Z, !out);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, true);
}

fn _swap(cpu: &mut CPU, v: u8) -> u8 {
    let high = v << 4;
    let low = v >> 4;

    let out = high | low;

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, false);

    out
}

/* Shifts v to the left. That is, the contents of bit 0 are copied to bit 1 and the previous
 * contents of bit 1 (the contents before the copy operation) are copied to bit 2.  The same
 * operation is repeated in sequence for the rest of the operand. The content of bit 7 is copied to
 * CY, and bit 0 is reset.
 */
fn _sla(cpu: &mut CPU, v: u8) -> u8 {
    let out = v << 1;

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

/* Shifts the contents of v to the right. That is, the contents of bit 7 are copied to bit
 * 6 and the previous contents of bit 6 (the contents before the copy operation) are copied to bit
 * 5. The same operation is repeated in sequence for the rest of the operand . The contents of bit
 * 0 are copied to CY, and the content of bit 7 is unchanged.
 */
fn _sra(cpu: &mut CPU, v: u8) -> u8 {
    let out = (v >> 1) | (v & 0b1000_0000);
    
    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Shifts the contents of v the right. That is, the contents of bit 7 are copied to bit
 * 6 and the previous contents of bit 6 (the contents before the copy operation) are copied to bit
 * 5. The same operation is repeated in sequence for the rest of the operand. The contents of bit
 * 0 are copied to CY, and bit 7 is reset.
 */
fn _srl(cpu: &mut CPU, v: u8) -> u8 {
    let out = v >> 1;

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotates the contents of operand m to the right.
 */
fn _rr(cpu: &mut CPU, v: u8) -> u8 {
    let c = cpu.registers.get_flag(Flag::C);

    let mut out = v >> 1;

    if c {
        out = out | 0x80;
    }

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n right. Old bit 0 to Carry flag.
 */
fn _rrc(cpu: &mut CPU, v: u8) -> u8 {
    let out = v.rotate_right(1);

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n left. Old bit 7 to Carry flag
 */
fn _rlc(cpu: &mut CPU, v: u8) -> u8 {
    let out = v.rotate_left(1);

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

/* Rotate left through c
 */
fn _rl(cpu: &mut CPU, v: u8) -> u8 {
    let c = cpu.registers.get_flag(Flag::C);

    let mut out = v << 1;

    if c {
        out = out | 0x01
    }

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

fn _or(cpu: &mut CPU, a:u8, b:u8) -> u8 {
    let v = a | b;

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, false);

    v
}

fn _xor(cpu: &mut CPU, a:u8, b:u8) -> u8 {
    let value = a ^ b;

    cpu.registers.set_flag(Flag::Z, value == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, false);

    value
}

fn _and(cpu: &mut CPU, a:u8, b:u8) -> u8 {
    let value = a & b;

    cpu.registers.set_flag(Flag::Z, value == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, true);
    cpu.registers.set_flag(Flag::C, false);

    value
}

fn _jr(cpu: &mut CPU, n:u8) {
    let pc = cpu.registers.get16(Registers16::PC);
    let (out, _overflow, _hc) = bytes::add_unsigned_signed(pc, n);
    cpu.registers.set16(Registers16::PC, out);
}


fn _add(cpu: &mut CPU, a: u8, b: u8) -> u8 {
    let (v, overflow) = a.overflowing_add(b);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry8(a, b));

    v
}

fn _add_u16_i8(cpu: &mut CPU, a: u16, b: u8) -> u16 {
    let (v, overflow, hc) = bytes::add_unsigned_signed(a, b);

    cpu.registers.set_flag(Flag::Z, false);
    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, hc);

    v
}

fn _adc(cpu: &mut CPU, a: u8, b: u8) -> u8 {
    let c = if cpu.registers.get_flag(Flag::C) { 1 } else { 0 };

    let (i, overflow1) = b.overflowing_add(c);
    let hc1 = bytes::check_half_carry8(b, c);

    let (v, overflow) = a.overflowing_add(i);
    let hc = bytes::check_half_carry8(a, i);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::C, overflow1 || overflow);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, hc1 || hc);

    v
}

fn _sub(cpu: &mut CPU, a: u8, b: u8) -> u8 {
    let (v, overflow) = a.overflowing_sub(b);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(a, b));

    v
}

fn _sbc(cpu: &mut CPU, a: u8, b: u8) -> u8 {
    let c = if cpu.registers.get_flag(Flag::C) { 1 } else { 0 };

    let (i, overflow1) = a.overflowing_sub(c);
    let hc1 = bytes::check_half_carry_sub8(a,c);

    let (v, overflow) = i.overflowing_sub(b);
    let hc = bytes::check_half_carry_sub8(i,b);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::C, overflow1 || overflow);
    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, hc1 || hc);

    v
}

fn _jump(cpu: &mut CPU, n: u16) {
    cpu.registers.set16(Registers16::PC, n);
}

fn _ret(cpu: &mut CPU) {
    _pop(cpu, Registers16::PC);
}

fn _pop(cpu: &mut CPU, r: Registers16) {
    let sp = cpu.registers.get16(Registers16::SP);
    let v = cpu.mmu.get16(sp);

    if r == Registers16::AF {
        /* Protect writing to F invalid values */
        cpu.registers.set16(r, v & 0xFFF0);
    } else {
        cpu.registers.set16(r, v);
    }

    cpu.registers.set16(Registers16::SP, sp + 2);
}

pub fn _call(cpu: &mut CPU, n: u16) {
    _push(cpu, Registers16::PC);
    _jump(cpu, n);
}

fn _push(cpu: &mut CPU, r: Registers16)  {
    let mut sp = cpu.registers.get16(Registers16::SP);

    let v = cpu.registers.get16(r);
    let (ms, ls) = bytes::split_ms_ls(v);

    sp = sp.wrapping_sub(1);
    cpu.mmu.set(sp, ls);

    sp = sp.wrapping_sub(1);
    cpu.mmu.set(sp, ms);

    cpu.registers.set16(Registers16::SP, sp);
}

pub struct OpResult {
    pub cycles: u8,
    pub name: String,
}

fn cycles(a: u8, n: String) -> OpResult {
    OpResult {
        cycles: a,
        name: n,
    }
}

/* Does nothing, pc advances 1
 */
pub fn nop(_cpu: &mut CPU) -> OpResult {
    cycles(4, "NOP".to_string())
}

/* Increment and Decrements */

/* Decrement register r
 */
pub fn dec_r8(cpu: &mut CPU, r:Registers8) -> OpResult {
    let i = cpu.registers.get8(r);

    let v = i.wrapping_sub(1);

    cpu.registers.set8(r, v);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(i, 1));

    cycles(4, format!("DEC R8 {:?}", r))
}

/* Decrement register r
 */
pub fn dec_r16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let i = cpu.registers.get16(r);

    let v = i.wrapping_sub(1);

    cpu.registers.set16(r, v);

    cycles(8, format!("DEC R16 {:?}", r))
}

/* Decrement memory pointed to by register r
 */
pub fn dec_ar16(cpu: &mut CPU, r:Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let i = cpu.mmu.get(address);

    let v = i.wrapping_sub(1);

    cpu.mmu.set(address, v);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(i, 1));


    cycles(12, format!("DEC AR16 {:?}", r))
}

/* Increment register r
 */
pub fn inc_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let i = cpu.registers.get8(r);
    let v = i.wrapping_add(1);

    cpu.registers.set8(r, v);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry8(i, 1));

    cycles(4, format!("INC R8 {:?}", r))
}

/* Incremenet register r
 */
pub fn inc_r16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let i = cpu.registers.get16(r);
    let v = i.wrapping_add(1);

    cpu.registers.set16(r, v);

    cycles(8, format!("INC R16 {:?}", r))
}



/* Incremenet memory pointed to by register r
 */
pub fn inc_ar16(cpu: &mut CPU, r:Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let i = cpu.mmu.get(address);
    let v = i.wrapping_add(1);

    cpu.mmu.set(address, v);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry8(i, 1));


    cycles(12, format!("INC AR16 {:?}", r))
}


/* Loads */

/* Loads a 8 bit value from r2 into the memory addressed by r1
 */
pub fn ld_ar16_r8(cpu: &mut CPU, r1: Registers16, r2: Registers8) -> OpResult {
    let address = cpu.registers.get16(r1);
    let value = cpu.registers.get8(r2);
    cpu.mmu.set(address, value);

    cycles(8, format!("LD AR16 R8 {:?} {:?}", r1, r2))
}

/* Loads a 8 bit immediate value into the memory addressed by r
 */
pub fn ld_ar16_n8(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.fetch_arg_8();
    cpu.mmu.set(address, value);
    cycles(12, format!("LD AR16 N8 {:?}", r))
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 */
pub fn ld_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let address = cpu.registers.get16(r2);
    let value = cpu.mmu.get(address);
    cpu.registers.set8(r1, value);
    cycles(8, format!("LD R8 AR16 {:?} {:?}", r1, r2))
}

/* Loads a 8 bit value from the memory addressed by a 16 bit immediate value into r1
 */
pub fn ld_r8_an16(cpu: &mut CPU, r: Registers8) -> OpResult {
    let address = cpu.fetch_arg_16();
    let value = cpu.mmu.get(address);
    cpu.registers.set8(r, value);
    cycles(16, format!("LD R8 AN16 {:?} {:X}", r, value))
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 * and simultaneously increments r2
 */
pub fn ldi_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let address = cpu.registers.get16(r2);
    let value = cpu.mmu.get(address);

    cpu.registers.set8(r1, value);
    cpu.registers.set16(r2, address.wrapping_add(1));

    cycles(8, format!("LDI R8 AR16 {:?} {:?}", r1, r2))
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 * and simultaneously decements r2
 */
pub fn ldd_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let address = cpu.registers.get16(r2);
    let value = cpu.mmu.get(address);

    cpu.registers.set8(r1, value);
    cpu.registers.set16(r2, address.wrapping_sub(1));
    cycles(8, format!("LDD R7 AR16 {:?} {:?}", r1, r2))
}

/* Loads a 8 bit value from r2 into the memory addressed by r1
 * and simultaneously increments r1
 */
pub fn ldi_ar16_r8(cpu: &mut CPU, r1: Registers16, r2: Registers8) -> OpResult {
    let address = cpu.registers.get16(r1);
    let value = cpu.registers.get8(r2);

    cpu.mmu.set(address, value);
    cpu.registers.set16(r1, address.wrapping_add(1));

    cycles(8, format!("LDI AR16 R8 {:?} {:?}", r1, r2))
}

/* Loads a 8 bit value from r2 into the memory addressed by r1
 * and simultaneously decrements r1
 */
pub fn ldd_ar16_r8(cpu: &mut CPU, r1: Registers16, r2: Registers8) -> OpResult {
    let address = cpu.registers.get16(r1);
    let value = cpu.registers.get8(r2);

    cpu.mmu.set(address, value);
    cpu.registers.set16(r1, address.wrapping_sub(1));
    cycles(8, format!("LDD AR16 R8 {:?} {:?}", r1, r2))
}

/* Loads a 8 bit immediate value into r
 */
pub fn ld_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.fetch_arg_8();
    cpu.registers.set8(r, value);
    cycles(8, format!("LD R8 N8 {:?} {:X}", r, value))
}

/* Loads a 8 bit value from r2 into r1
 */
pub fn ld_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let value = cpu.registers.get8(r2);
    cpu.registers.set8(r1, value);
    cycles(4, format!("LD R8 R8 {:?} {:?}", r1, r2))
}

/* Loads a 16 bit value from r into the the memory addressed by a 16 bit immediate value
 */
pub fn ld_an16_r16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let value = cpu.registers.get16(r);
    let address = cpu.fetch_arg_16();
    cpu.mmu.set16(address, value);
    cycles(20, format!("LD AN16 R16 {:X} {:?}", value, r))
}

/* Loads an 8 bit value from r into the the memory addressed by a 16 bit immediate value 
 */
pub fn ld_an16_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let address = cpu.fetch_arg_16();
    cpu.mmu.set(address, value);
    cycles(16, format!("LD AN16 R8 {:X}, {:?}", value, r))
}

/* Loads a 16 bit value from args into the register r
 */
pub fn ld_r16_n16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let value = cpu.fetch_arg_16();
    cpu.registers.set16(r, value);
    cycles(12, format!("LD R16 N16 {:?} {:X}", r, value))
}

/* Loads a 16 bit value from r1 into r2
 */
pub fn ld_r16_r16(cpu: &mut CPU, r1: Registers16, r2: Registers16) -> OpResult {
    let value = cpu.registers.get16(r1);
    cpu.registers.set16(r2, value);
    cycles(12, format!("LD R16 R16 {:?} {:?}", r1, r2))
}

pub fn ld_r16_spn8(cpu: &mut CPU, r: Registers16) -> OpResult {
    let a = cpu.registers.get16(Registers16::SP);
    let b = cpu.fetch_arg_8();

    let v = _add_u16_i8(cpu, a, b);

    cpu.registers.set16(r, v);

    cycles(12, format!("LD R16 SPN8 {:?} {:X}", r, b))
}

/* Loads an 8 bit value from r value into the memory at FF00 + an
 */
pub fn ldh_an8_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let v = cpu.registers.get8(r);
    let an = cpu.fetch_arg_8() as u16;
    cpu.mmu.set(0xFF00 + an, v);
    cycles(12, format!("LDH AN8 R8 {:X}, {:?}", an, r))
}

pub fn ldh_r8_an8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let an = cpu.fetch_arg_8() as u16;
    let v = cpu.mmu.get(0xFF00 + an);

    cpu.registers.set8(r, v);
    cycles(12, format!("LDH R8 AN8 {:?} {:X}", r, an))
}

pub fn ldc_ar8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let ar = cpu.registers.get8(r1) as u16;
    let v = cpu.registers.get8(r2);

    cpu.mmu.set(0xFF00 + ar, v);
    cycles(12, format!("LDC AR8 R8 {:?} {:?}", r1, r2))
}

pub fn ldc_r8_ar8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let ar = cpu.registers.get8(r2) as u16;
    let v = cpu.mmu.get(0xFF00 + ar);

    cpu.registers.set8(r1, v);
    cycles(12, format!("LDC R8 AR8 {:?} {:?}", r1, r2))
}

/* Shifts and Rotates */

/* Rotates the A register left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rla(cpu: &mut CPU) -> OpResult {
    let value = cpu.registers.get8(Registers8::A);
    let out = _rl(cpu, value);
    cpu.registers.set8(Registers8::A, out);
    cpu.registers.set_flag(Flag::Z, false);
    cycles(4, "RLA".to_string())
}

/* Rotates the register r left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rl_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _rl(cpu, value);
    cpu.registers.set8(r, out);
    cycles(8, format!("RL R8 {:?}", r))
}

/* Rotates memory addressed by register r left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rl_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _rl(cpu, value);
    cpu.mmu.set(address, out);
    cycles(16, format!("RL AR16 {:?}", r))
}

/* Rotates the A register right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrca(cpu: &mut CPU) -> OpResult {
    let value = cpu.registers.get8(Registers8::A);
    let out = _rrc(cpu, value);
    cpu.registers.set8(Registers8::A, out);
    cpu.registers.set_flag(Flag::Z, false);
    cycles(4, "RRCA".to_string())
}

/* Rotates the register r right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrc_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _rrc(cpu, value);
    cpu.registers.set8(r, out);
    cycles(8, format!("RRC R8 {:?}", r))
}

/* Rotates the memory pointed to by r16 right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrc_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _rrc(cpu, value);
    cpu.mmu.set(address, out);
    cycles(16, format!("RRC AR16 {:?}", r))
}

/* Rotates the A register right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rra(cpu: &mut CPU) -> OpResult {
    let value = cpu.registers.get8(Registers8::A);
    let out = _rr(cpu, value);
    cpu.registers.set8(Registers8::A, out);
    cpu.registers.set_flag(Flag::Z, false);
    cycles(4, "RRA".to_string())
}

/* Rotates the register r right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rr_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _rr(cpu, value);
    cpu.registers.set8(r, out);
    cycles(9, format!("RR R8: {:?}", r))
}

/* Rotates the memory pointed to from r right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rr_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _rr(cpu, value);
    cpu.mmu.set(address, out);
    cycles(16, format!("RR AR16 {:?}", r))
}

/* Rotates the A register left, puts the shifted bit in c
 * If you have C=1 00010001 and call RLCA the result is C=0 00100010
 * the left most bit is shifted onto C but isn't rotated
 */
pub fn rlca(cpu: &mut CPU) -> OpResult {
    let value = cpu.registers.get8(Registers8::A);
    let out = _rlc(cpu, value);
    cpu.registers.set8(Registers8::A, out);
    cpu.registers.set_flag(Flag::Z, false);
    cycles(4, "RLCA".to_string())
}

pub fn rlc_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _rlc(cpu, value);
    cpu.registers.set8(r, out);
    cycles(8, format!("RLC R8 {:?}", r))
}

pub fn rlc_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _rlc(cpu, value);
    cpu.mmu.set(address, out);
    cycles(16, format!("RLC AR16 {:?}", r))
}

/* Shift the contents of register r left into Carry. LSB of n set to 0.
 */
pub fn sla_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _sla(cpu, value);
    cpu.registers.set8(r, out);
    cycles(8, format!("SLA R8 {:?}", r))
}

/* Shift the memory addressed by r left into Carry. LSB of n set to 0.
 */
pub fn sla_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _sla(cpu, value);
    cpu.mmu.set(address, out);
    cycles(16, format!("SLA AR16 {:?}", r))
}

/* Shift the contents of register r right into Carry. LSB of n set to 0.
 */
pub fn sra_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _sra(cpu, value);
    cpu.registers.set8(r, out);
    cycles(8, format!("SRA R8 {:?}", r))
}

/* Shift the memory addressed by r right into Carry. LSB of n set to 0.
 */
pub fn sra_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _sra(cpu, value);
    cpu.mmu.set(address, out);
    cycles(16 , format!("SRA AR16 {:?}", r))
}

/* Halt CPU & LCD display until button pressed
 */
pub fn stop(cpu: &mut CPU) -> OpResult {
    cpu.stop();
    cycles(4, "STOP".to_string())
}

/* Halt CPU & LCD display until button pressed
 */
pub fn halt(cpu: &mut CPU) -> OpResult {
    cpu.halt();
    cycles(4, "HALT".to_string())
}

/* Jumps */

pub fn jp_f_n16(cpu: &mut CPU, f: JumpFlag) -> OpResult {
    let n = cpu.fetch_arg_16();

    /* TODO: note there is a difference in cycle count
     * between matching and not matching branches
     */
    match f {
        JumpFlag::NZ => {
            if !cpu.registers.get_flag(Flag::Z) {
                _jump(cpu, n);
            }
        },
        JumpFlag::Z => {
            if cpu.registers.get_flag(Flag::Z) {
                _jump(cpu, n);
            }
        },
        JumpFlag::NC => {
            if !cpu.registers.get_flag(Flag::C) {
                _jump(cpu, n);
            }
        }
        JumpFlag::C => {
            if cpu.registers.get_flag(Flag::C) {
                _jump(cpu, n);
            }
        }
    }

    cycles(12, format!("JP F N16 {:?} {:X}", f, n))
}

pub fn jp_n16(cpu: &mut CPU) -> OpResult {
    let n = cpu.fetch_arg_16();
    _jump(cpu, n);
    cycles(12, format!("JP N16 {:X}", n))
}

pub fn jp_r16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let n = cpu.registers.get16(r);
    _jump(cpu, n);
    cycles(4, format!("JP AR16 {:?}", r))
}

pub fn call_n16(cpu: &mut CPU) -> OpResult {
    let v = cpu.fetch_arg_16();
    _push(cpu, Registers16::PC);
    _jump(cpu, v);
    cycles(12, format!("CALL N16 {:X}", v))
}

pub fn call_f_n16(cpu: &mut CPU, f: JumpFlag) -> OpResult {
    let n = cpu.fetch_arg_16();

    match f {
        JumpFlag::NZ => {
            if !cpu.registers.get_flag(Flag::Z) {
                _call(cpu, n);
            }
        },
        JumpFlag::Z => {
            if cpu.registers.get_flag(Flag::Z) {
                _call(cpu, n);
            }
        },
        JumpFlag::NC => {
            if !cpu.registers.get_flag(Flag::C) {
                _call(cpu, n);
            }
        }
        JumpFlag::C => {
            if cpu.registers.get_flag(Flag::C) {
                _call(cpu, n);
            }
        }
    }

    cycles(12, format!("CALL F N16 {:?} {:X}", f, n))
}

pub fn push_r16(cpu: &mut CPU, r: Registers16) -> OpResult {
    _push(cpu, r);
    cycles(16, format!("PUSH R16 {:?}", r))
}

pub fn rst_f(cpu: &mut CPU, f: RstFlag) -> OpResult {
    let location = rst_locations(f);
    _push(cpu, Registers16::PC);
    _jump(cpu, location as u16);
    cycles(32, format!("RST F {:?}", f))

}

pub fn di(cpu: &mut CPU) -> OpResult {
    cpu.registers.interrupts_enabled = false;
    cycles(4, "DI".to_string())
}

pub fn ei(cpu: &mut CPU) -> OpResult {
    cpu.registers.interrupts_enabled = true;
    cycles(4, "EI".to_string())
}

/* Pop two bytes from stack & jump to that address
 *
 * Note: Jumping is just setting the PC register
 * So we can simplify this function by just passing PC
 * to the _pop function that takes the values from the stack
 * and sets them to the given register.
 */
pub fn ret(cpu: &mut CPU) -> OpResult {
    _ret(cpu);
    cycles(8, "RET".to_string())
}

pub fn reti(cpu: &mut CPU) -> OpResult {
    _ret(cpu);
    cpu.registers.interrupts_enabled = true;
    cycles(8, "RETI".to_string())
}

pub fn ret_f(cpu: &mut CPU, f: JumpFlag) -> OpResult {
    match f {
        JumpFlag::NZ => {
            if !cpu.registers.get_flag(Flag::Z) {
                _ret(cpu);
            }
        },
        JumpFlag::Z => {
            if cpu.registers.get_flag(Flag::Z) {
                _ret(cpu);
            }
        },
        JumpFlag::NC => {
            if !cpu.registers.get_flag(Flag::C) {
                _ret(cpu);
            }
        }
        JumpFlag::C => {
            if cpu.registers.get_flag(Flag::C) {
                _ret(cpu);
            }
        }
    }

    cycles(8, format!("RET F {:?}", f))
}


pub fn pop_r16(cpu: &mut CPU, r: Registers16) -> OpResult {
    _pop(cpu, r);
    cycles(12, format!("POP R16 {:?}", r))
}

pub fn jr_n8(cpu: &mut CPU) -> OpResult {
    let n = cpu.fetch_arg_8();
    _jr(cpu, n);
    cycles(8, format!("JR N8 {:X}", n))
}


pub fn jr_f_n8(cpu: &mut CPU, f: JumpFlag) -> OpResult {
    let n = cpu.fetch_arg_8();

    match f {
        JumpFlag::NZ => {
            if !cpu.registers.get_flag(Flag::Z) {
                _jr(cpu, n);
            }
        },
        JumpFlag::Z => {
            if cpu.registers.get_flag(Flag::Z) {
                _jr(cpu, n);
            }
        },
        JumpFlag::NC => {
            if !cpu.registers.get_flag(Flag::C) {
                _jr(cpu, n);
            }
        }
        JumpFlag::C => {
            if cpu.registers.get_flag(Flag::C) {
                _jr(cpu, n);
            }
        }
    }

    cycles(8, format!("JR F N8 {:?} {:X}", f, n))
}

/* Applies the binary complement to the A register
 */
pub fn cpl(cpu: &mut CPU) -> OpResult {
    let v = cpu.registers.get8(Registers8::A);
    cpu.registers.set8(Registers8::A, !v);

    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, true);

    cycles(4, "CPL".to_string())
}

/* When performing addition and subtraction, binary coded decimal
 * representation is used to set the contents of register A to a binary coded
 * decimal number (BCD).
 *
 * BCD represents the top and bottom nibbles of an 8bit number as individual
 * decimal numerals.
 *
 * For example 49 would be represented as
 *
 * 0100_1001
 *
 * the DAA function thus takes the A register and maps its value into BCD
 *
*/ 
pub fn daa(cpu: &mut CPU) -> OpResult {
    let mut v = cpu.registers.get8(Registers8::A);

    let n = cpu.registers.get_flag(Flag::N);
    let h = cpu.registers.get_flag(Flag::H);
    let c = cpu.registers.get_flag(Flag::C);

    if n {
        if c {
            v = v.wrapping_sub(0x60);
        }
        if h {
            v = v.wrapping_sub(0x06);
        }
    } else {
        if c || v > 0x99 {
            v = v.wrapping_add(0x60);
            cpu.registers.set_flag(Flag::C, true);
        }

        if h || (v & 0x0F) > 9 {
            v = v.wrapping_add(0x06);
        }
    }

    cpu.registers.set8(Registers8::A, v);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::H, false);

    cycles(4, "DAA".to_string())
}

pub fn scf(cpu: &mut CPU) -> OpResult {

    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, true);

    cycles(4, "SCF".to_string())
}

pub fn ccf(cpu: &mut CPU) -> OpResult {
    let c = cpu.registers.get_flag(Flag::C);

    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, !c);

    cycles(4, "CCF".to_string())
}


pub fn add_r16_n8(cpu: &mut CPU, r: Registers16) -> OpResult {
    let a = cpu.registers.get16(r);
    let b = cpu.fetch_arg_8();


    let v = _add_u16_i8(cpu, a, b);

    cpu.registers.set16(r, v);

    cycles(16, format!("ADD R16 N8 {:?} {:X}", r, b))
}

pub fn add_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _add(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(4, format!("ADD R8 R8 {:?} {:?}", r1, r2))
}

pub fn add_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _add(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("ADD R8 N8 {:?} {:X}", r, b))
}

pub fn add_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _add(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("ADD R8 AR16 {:?} {:?}", r1, r2))
}

pub fn adc_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _adc(cpu, a, b);

    cpu.registers.set8(r1, v);

    cycles(4, format!("ADC R8 R8 {:?} {:?}", r1, r2))
}

pub fn adc_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _adc(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("ADC R8 AR16 {:?} {:?}", r1, r2))
}

pub fn adc_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _adc(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("ADC R8 N8 {:?}", r))
}

pub fn add_r16_r16(cpu: &mut CPU, r1: Registers16, r2: Registers16) -> OpResult {
    let a = cpu.registers.get16(r1);
    let b = cpu.registers.get16(r2);

    let (v, overflow) = a.overflowing_add(b);

    cpu.registers.set16(r1, v);

    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry16(a, b));

    cycles(8, format!("ADD R16 R16 {:?} {:?}", r1, r2))
}


pub fn sub_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _sub(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(4, format!("SUB R8 R8 {:?} {:?}", r1, r2))
}

pub fn sub_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _sub(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("SUB R8 AR16 {:?} {:?}", r1, r2))
}

pub fn sub_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _sub(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("SUB R8 N8 {:?}", r))
}

pub fn sbc_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _sbc(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(4, format!("SBC R8 R8 {:?} {:?}", r1, r2))
}

pub fn sbc_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _sbc(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("SBC R8 R16 {:?} {:?}", r1, r2))
}

pub fn sbc_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _sbc(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("SBC R8 N8 {:?}", r))
}

pub fn and_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _and(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("ADD R8 N8 {:?}", r))
}

pub fn and_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _and(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(4, format!("ADD R8 R8 {:?} {:?}", r1, r2))
}

pub fn and_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _and(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("AND R8 AR16 {:?} {:?}", r1, r2))
}

pub fn xor_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _xor(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(4, format!("XOR R8 R8 {:?} {:?}", r1, r2))
}

pub fn xor_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _xor(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("XOR R8 N8 {:?}", r))
}

pub fn xor_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _xor(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("XOR R8 AR16 {:?} {:?}", r1, r2))
}

pub fn or_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    let v = _or(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(4, format!("OX R8 R8 {:?} {:?}", r1, r2))
}

pub fn or_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    let v = _or(cpu, a, b);

    cpu.registers.set8(r, v);
    cycles(8, format!("OR R8 N8 {:?}", r))
}

pub fn or_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    let v = _or(cpu, a, b);

    cpu.registers.set8(r1, v);
    cycles(8, format!("OR R8 AR16 {:?} {:?}", r1, r2))
}

pub fn cp_r8_r8(cpu: &mut CPU, r1: Registers8, r2: Registers8) -> OpResult {
    let a = cpu.registers.get8(r1);
    let b = cpu.registers.get8(r2);

    _sub(cpu, a, b);

    cycles(4, format!("CP R8 R8 {:?} {:?}", r1, r2))
}

pub fn cp_r8_n8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let a = cpu.registers.get8(r);
    let b = cpu.fetch_arg_8();

    _sub(cpu, a, b);

    cycles(8, format!("CP R8 N8 {:?} {:X}", r, b))
}

pub fn cp_r8_ar16(cpu: &mut CPU, r1: Registers8, r2: Registers16) -> OpResult {
    let a = cpu.registers.get8(r1);
    let address = cpu.registers.get16(r2);
    let b = cpu.mmu.get(address);

    _sub(cpu, a, b);

    cycles(8, format!("CP R8 AR16 {:?} {:?}", r1, r2))
}

pub fn swap_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _swap(cpu, value);
    cpu.registers.set8(r, out);
    cycles(8, format!("SWAP {:?}", r))
}

pub fn swap_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _swap(cpu, value);
    cpu.mmu.set(address, out);

    cycles(16, format!("SWAP {:?}", r))
}

pub fn srl_r8(cpu: &mut CPU, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);
    let out = _srl(cpu, value);
    cpu.registers.set8(r, out);

    cycles(8, format!("SRC R8 {:?}", r))
}

pub fn srl_ar16(cpu: &mut CPU, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);
    let out = _srl(cpu, value);
    cpu.mmu.set(address, out);

    cycles(16, format!("SRL AR16 {:?}", r))
}

pub fn bit_r8(cpu: &mut CPU, n:u8, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);

    _bit(cpu, n, value);

    cycles(8, format!("BIT R8 {:?} {:?}", n, r))
}

pub fn bit_ar16(cpu: &mut CPU, n:u8, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);

    _bit(cpu, n, value);

    cycles(16, format!("BIT AR16 {:?} {:?}", n, r))
}

pub fn res_r8(cpu: &mut CPU, n:u8, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);

    let out = _res(n, value);

    cpu.registers.set8(r, out);

    cycles(8, format!("RES R8 {:?} {:?}", n, r))
}

pub fn res_ar16(cpu: &mut CPU, n:u8, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);

    let out = _res(n, value);

    cpu.mmu.set(address, out);

    cycles(16, format!("RES AR16 {:?} {:?}", n, r))
}

pub fn set_r8(cpu: &mut CPU, n:u8, r: Registers8) -> OpResult {
    let value = cpu.registers.get8(r);

    let out = _set(n, value);

    cpu.registers.set8(r, out);

    cycles(8, format!("SET R8 {:?} {:?}", n, r))
}

pub fn set_ar16(cpu: &mut CPU, n:u8, r: Registers16) -> OpResult {
    let address = cpu.registers.get16(r);
    let value = cpu.mmu.get(address);

    let out = _set(n, value);

    cpu.mmu.set(address, out);

    cycles(16, format!("SET AR16 {:?} {:?}", n, r))
}

pub fn illegal_opcode(opcode: &str) -> OpResult {
    panic!("attempted to call: {}", opcode);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::rom::BootRom;
    use crate::cartridge::Cartridge;
    use crate::mmu::MMU;

    #[test]
    fn test_bit_r8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        _bit(&mut cpu, 7, 0x80);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
    }

    #[test]
    fn test_set_r8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0x80);
        set_r8(&mut cpu, 3, Registers8::A);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x88);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_scf() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        scf(&mut cpu);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_adc_r8_r8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0xE1);
        cpu.registers.set8(Registers8::E, 0x0F);
        cpu.registers.set_flag(Flag::C, true);

        adc_r8_r8(&mut cpu, Registers8::A, Registers8::E);

        assert_eq!(cpu.registers.get8(Registers8::A), 0xF1);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_adc_r8_n8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0xE1);
        cpu.registers.set_flag(Flag::C, true);
        cpu.push_pc(0xFF80, 0x3B);

        adc_r8_n8(&mut cpu, Registers8::A);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x1D);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_sdc_r8_r8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0x3B);
        cpu.registers.set8(Registers8::H, 0x2A);
        cpu.registers.set_flag(Flag::C, true);

        sbc_r8_r8(&mut cpu, Registers8::A, Registers8::H);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x10);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_dec_r8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::L, 0x01);

        dec_r8(&mut cpu, Registers8::L);

        assert_eq!(cpu.registers.get8(Registers8::L), 0x00);

        assert_eq!(cpu.registers.get_flag(Flag::Z), true);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        // interestingly blarggs says inc and dec nevery set carry
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_inc_r8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::L, 0x01);

        inc_r8(&mut cpu, Registers8::L);

        assert_eq!(cpu.registers.get8(Registers8::L), 0x02);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_rlca() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0x85);

        rlca(&mut cpu);

        /* Note this disagrees with the gamboy manual
         * The manual suggests this should be 0x0A but that
         * seems impossible.
         */
        assert_eq!(cpu.registers.get8(Registers8::A), 0x0B);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_sra() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0x8A);

        sra_r8(&mut cpu, Registers8::A);

        assert_eq!(cpu.registers.get8(Registers8::A), 0xC5);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_dec_ar16() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.mmu.set(0xFF80, 0x00);
        cpu.registers.set16(Registers16::HL, 0xFF80);
        cpu.registers.set_flag(Flag::C, true);

        dec_ar16(&mut cpu, Registers16::HL);

        assert_eq!(cpu.mmu.get(0xFF80), 0xFF);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);

        /* assert that C isn't affected */
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_inc_ar16() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.mmu.set(0xFF80, 0x50);
        cpu.registers.set16(Registers16::HL, 0xFF80);
        cpu.registers.set_flag(Flag::C, true);

        inc_ar16(&mut cpu, Registers16::HL);

        assert_eq!(cpu.mmu.get(0xFF80), 0x51);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);

        /* assert that C isn't affected */
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_daa() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set8(Registers8::A, 0x45);
        cpu.registers.set8(Registers8::B, 0x38);
        cpu.registers.set_flag(Flag::N, true);

        add_r8_r8(&mut cpu, Registers8::A, Registers8::B);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x7D);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);

        daa(&mut cpu);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x83);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

        sub_r8_r8(&mut cpu, Registers8::A, Registers8::B);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x4B);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);

        daa(&mut cpu);

        assert_eq!(cpu.registers.get8(Registers8::A), 0x45);
    }

    #[test]
    fn test_ld_r16_spn8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set16(Registers16::SP, 0xFFF8);
        cpu.push_pc(0xFF80, 0x02);

        ld_r16_spn8(&mut cpu, Registers16::HL);

        assert_eq!(cpu.registers.get16(Registers16::HL), 0xFFFA);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

    }

    #[test]
    fn test_ld_r16_spn8_sub() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set16(Registers16::SP, 0xDFFD);
        cpu.push_pc(0xFF80, 0xFE);

        ld_r16_spn8(&mut cpu, Registers16::HL);

        assert_eq!(cpu.registers.get16(Registers16::HL), 0xDFFB);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

    }

    #[test]
    fn test_add_r16_n8() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set16(Registers16::SP, 0xDFFD);
        cpu.push_pc(0xFF80, 0x01);

        add_r16_n8(&mut cpu, Registers16::SP);

        assert_eq!(cpu.registers.get16(Registers16::SP), 0xDFFE);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_add_r16_n8_sub() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set16(Registers16::SP, 0xDFFD);
        cpu.push_pc(0xFF80, 0xFF);

        add_r16_n8(&mut cpu, Registers16::SP);

        assert_eq!(cpu.registers.get16(Registers16::SP), 0xDFFC);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_jp_f_n16() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set_flag(Flag::Z, true);
        cpu.push_pc(0xFF80, 0x10);

        jp_f_n16(&mut cpu, JumpFlag::Z);

        assert_eq!(cpu.registers.get16(Registers16::PC), 0x10);
    }

    #[test]
    fn test_jp_f_n16_no_test() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        cpu.registers.set_flag(Flag::Z, true);
        cpu.push_pc(0xFF80, 0x10);

        jp_f_n16(&mut cpu, JumpFlag::NZ);

        assert_eq!(cpu.registers.get16(Registers16::PC), 0xFF82);
    }

    #[test]
    fn test_call() {
        let mut cpu = CPU::new(MMU::new(BootRom::zero(), Cartridge::zero()));

        //cpu.push_pc(0x8000, 0x1234);
        cpu.push_pc(0x8001, 0x12);
        cpu.push_pc(0x8000, 0x34);
        cpu.registers.set16(Registers16::SP, 0xFFFE);

        call_n16(&mut cpu);

        assert_eq!(cpu.registers.get16(Registers16::PC), 0x1234);
        assert_eq!(cpu.registers.get16(Registers16::SP), 0xFFFC);

        assert_eq!(cpu.mmu.get(0xFFFD), 0x80);
        assert_eq!(cpu.mmu.get(0xFFFC), 0x03);
    }
}

