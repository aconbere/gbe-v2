use crate::cpu::CPU;
use crate::bytes;
use crate::register::{Flag, Registers16};


/* Helper Functions */

pub fn set(location: u8, v: u8) -> u8 {
    v | (1 << location)
}

/* Resets to 0 the specified bit in the specified register r
 */
pub fn res(location: u8, v: u8) -> u8 {
    v & !(1 << location)
}

/* Copies the complement of the contents of the specified bit in register r to the Z flag of the
 * program status word (PSW).
*/
pub fn bit(cpu: &mut CPU, location:u8, v:u8) {
    let out = bytes::check_bit(v, location);

    cpu.registers.set_flag(Flag::Z, !out);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, true);
}

pub fn swap(cpu: &mut CPU, v: u8) -> u8 {
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
pub fn sla(cpu: &mut CPU, v: u8) -> u8 {
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
pub fn sra(cpu: &mut CPU, v: u8) -> u8 {
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
pub fn srl(cpu: &mut CPU, v: u8) -> u8 {
    let out = v >> 1;

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotates the contents of operand m to the right.
 */
pub fn rr(cpu: &mut CPU, v: u8) -> u8 {
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
pub fn rrc(cpu: &mut CPU, v: u8) -> u8 {
    let out = v.rotate_right(1);

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n left. Old bit 7 to Carry flag
 */
pub fn rlc(cpu: &mut CPU, v: u8) -> u8 {
    let out = v.rotate_left(1);

    cpu.registers.set_flag(Flag::Z, out == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

/* Rotate left through c
 */
pub fn rl(cpu: &mut CPU, v: u8) -> u8 {
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

pub fn or(cpu: &mut CPU, a:u8, b:u8) -> u8 {
    let v = a | b;

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, false);

    v
}

pub fn xor(cpu: &mut CPU, a:u8, b:u8) -> u8 {
    let value = a ^ b;

    cpu.registers.set_flag(Flag::Z, value == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, false);
    cpu.registers.set_flag(Flag::C, false);

    value
}

pub fn and(cpu: &mut CPU, a:u8, b:u8) -> u8 {
    let value = a & b;

    cpu.registers.set_flag(Flag::Z, value == 0);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, true);
    cpu.registers.set_flag(Flag::C, false);

    value
}

pub fn jr(cpu: &mut CPU, n:u8) {
    let pc = cpu.registers.get16(Registers16::PC);
    let (out, _overflow, _hc) = bytes::add_unsigned_signed(pc, n);
    cpu.registers.set16(Registers16::PC, out);
}


pub fn add(cpu: &mut CPU, a: u8, b: u8) -> u8 {
    let (v, overflow) = a.overflowing_add(b);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry8(a, b));

    v
}

pub fn add_u16_i8(cpu: &mut CPU, a: u16, b: u8) -> u16 {
    let (v, overflow, hc) = bytes::add_unsigned_signed(a, b);

    cpu.registers.set_flag(Flag::Z, false);
    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, false);
    cpu.registers.set_flag(Flag::H, hc);

    v
}

pub fn adc(cpu: &mut CPU, a: u8, b: u8) -> u8 {
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

pub fn sub(cpu: &mut CPU, a: u8, b: u8) -> u8 {
    let (v, overflow) = a.overflowing_sub(b);

    cpu.registers.set_flag(Flag::Z, v == 0);
    cpu.registers.set_flag(Flag::C, overflow);
    cpu.registers.set_flag(Flag::N, true);
    cpu.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(a, b));

    v
}

pub fn sbc(cpu: &mut CPU, a: u8, b: u8) -> u8 {
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

pub fn jump(cpu: &mut CPU, n: u16) {
    cpu.registers.set16(Registers16::PC, n);
}

pub fn ret(cpu: &mut CPU) {
    pop(cpu, Registers16::PC);
}

pub fn pop(cpu: &mut CPU, r: Registers16) {
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

pub struct Call {
    function: u16,
    from: u16
}

pub fn call(cpu: &mut CPU, n: u16) {
    cpu.push_call(
        cpu.registers.pc
    );
    push(cpu, Registers16::PC);
    jump(cpu, n);
}

pub fn push(cpu: &mut CPU, r: Registers16)  {
    let mut sp = cpu.registers.get16(Registers16::SP);

    let v = cpu.registers.get16(r);
    let (ms, ls) = bytes::split_ms_ls(v);

    sp = sp.wrapping_sub(1);
    cpu.mmu.set(sp, ms);

    sp = sp.wrapping_sub(1);
    cpu.mmu.set(sp, ls);

    cpu.registers.set16(Registers16::SP, sp);
}
