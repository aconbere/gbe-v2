use crate::register::{Registers16, Registers8, Flag};
use crate::gameboy::Gameboy;
use crate::bytes;

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

pub enum JumpFlag {
    NZ, Z, NC, C
}

/* Helper Functions */

fn _set(location: u8, v: u8) -> u8 {
    v & (1 << location)
}

/* Resets to 0 the specified bit in the specified register r
 */
fn _res(location: u8, v: u8) -> u8 {
    v & !(1 << location)
}

/* Copies the complement of the contents of the specified bit in register r to the Z flag of the
 * program status word (PSW).
*/
fn _bit(gameboy: &mut Gameboy, location:u8, v:u8) {
    let out = bytes::check_bit(v, location);

    gameboy.registers.set_flag(Flag::Z, !out);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
}

fn _swap(gameboy: &mut Gameboy, v: u8) -> u8 {
    let high = v << 4;
    let low = v >> 4;

    let out = high | low;

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, false);

    out
}

/* Shifts v to the left. That is, the contents of bit 0 are copied to bit 1 and the previous
 * contents of bit 1 (the contents before the copy operation) are copied to bit 2.  The same
 * operation is repeated in sequence for the rest of the operand. The content of bit 7 is copied to
 * CY, and bit 0 is reset.
 */
fn _sla(gameboy:&mut Gameboy, v: u8) -> u8 {
    let out = v << 1;

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

/* Shifts the contents of v to the right. That is, the contents of bit 7 are copied to bit
 * 6 and the previous contents of bit 6 (the contents before the copy operation) are copied to bit
 * 5. The same operation is repeated in sequence for the rest of the operand . The contents of bit
 * 0 are copied to CY, and the content of bit 7 is unchanged.
 */
fn _sra(gameboy:&mut Gameboy, v: u8) -> u8 {
    let c = bytes::check_bit(v, 7);

    let mut out = v >> 1;
    
    bytes::set_bit(out, 7, c);

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Shifts the contents of v the right. That is, the contents of bit 7 are copied to bit
 * 6 and the previous contents of bit 6 (the contents before the copy operation) are copied to bit
 * 5. The same operation is repeated in sequence for the rest of the operand. The contents of bit
 * 0 are copied to CY, and bit 7 is reset.
 */
fn _srl(gameboy:&mut Gameboy, v: u8) -> u8 {
    let out = v >> 1;

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotates the contents of operand m to the right.
 */
fn _rr(gameboy: &mut Gameboy, v: u8) -> u8 {
    let c = gameboy.registers.get_flag(Flag::C);

    let mut out = v >> 1;

    if c {
        out = out | 0x80;
    }

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n right. Old bit 0 to Carry flag.
 */
fn _rrc(gameboy: &mut Gameboy, v: u8) -> u8 {
    let out = v.rotate_right(1);

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n left. Old bit 7 to Carry flag
 */
fn _rlc(gameboy: &mut Gameboy, v: u8) -> u8 {
    let out = v.rotate_left(1);

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

/* Rotate left through c
 */
fn _rl(gameboy: &mut Gameboy, v: u8) -> u8 {
    let c = gameboy.registers.get_flag(Flag::C);

    let mut out = v << 1;

    if c {
        out = out | 0x01
    }

    gameboy.registers.set_flag(Flag::Z, out == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

fn _or(gameboy: &mut Gameboy, a:u8, b:u8) -> u8 {
    let v = a | b;

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, false);

    v
}

fn _xor(gameboy: &mut Gameboy, a:u8, b:u8) -> u8 {
    let value = a ^ b;

    gameboy.registers.set_flag(Flag::Z, value == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, false);

    value
}

fn _and(gameboy: &mut Gameboy, a:u8, b:u8) -> u8 {
    let value = a & b;

    gameboy.registers.set_flag(Flag::Z, value == 0);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, true);
    gameboy.registers.set_flag(Flag::C, false);

    value
}

fn _jr(gameboy: &mut Gameboy, n:u8) {
    let pc = gameboy.registers.get16(Registers16::PC);
    let (out, _overflow, _hc) = bytes::add_unsigned_signed(pc, n);
    gameboy.registers.set16(Registers16::PC, out);
}


pub fn _add(gameboy: &mut Gameboy, a: u8, b: u8) -> u8 {
    let (v, overflow) = a.overflowing_add(b);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry8(a, b));

    v
}

pub fn _add_u16_i8(gameboy: &mut Gameboy, a: u16, b: u8) -> u16 {
    let (v, overflow, hc) = bytes::add_unsigned_signed(a, b);

    gameboy.registers.set_flag(Flag::Z, false);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, hc);

    v
}

pub fn _adc(gameboy: &mut Gameboy, a: u8, b: u8) -> u8 {
    let c = if gameboy.registers.get_flag(Flag::C) { 1 } else { 0 };

    let (i, overflow1) = b.overflowing_add(c);
    let (v, overflow) = a.overflowing_add(i);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow1 || overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry8(a, b));

    v
}

pub fn _sub(gameboy: &mut Gameboy, a: u8, b: u8) -> u8 {
    let (v, overflow) = a.overflowing_sub(b);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, true);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(a, b));

    v
}

pub fn _sbc(gameboy: &mut Gameboy, a: u8, b: u8) -> u8 {
    let c = if gameboy.registers.get_flag(Flag::C) { 1 } else { 0 };

    let (i, overflow1) = b.overflowing_sub(c);
    let (v, overflow) = a.overflowing_sub(i);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow1 || overflow);
    gameboy.registers.set_flag(Flag::N, true);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(a, b));

    v
}

pub fn _jump(gameboy: &mut Gameboy, n: u16) {
    gameboy.registers.set16(Registers16::PC, n);
}

pub fn _ret(gameboy: &mut Gameboy) {
    _pop(gameboy, Registers16::PC);
}

pub fn _pop(gameboy: &mut Gameboy, r: Registers16) {
    let sp = gameboy.registers.get16(Registers16::SP);
    let v = gameboy.mmu.get16(sp);
    gameboy.registers.set16(r, v);
    gameboy.registers.set16(Registers16::SP, sp + 2);
}

pub fn _call(gameboy: &mut Gameboy, n: u16) {
    push_r16(gameboy, Registers16::PC);
    _jump(gameboy, n);
}

pub fn _push(gameboy: &mut Gameboy, r: Registers16)  {
    let mut sp = gameboy.registers.get16(Registers16::SP);

    let v = gameboy.registers.get16(r);
    let (ms, ls) = bytes::split_ms_ls(v);

    sp = sp.wrapping_sub(1);
    gameboy.mmu.set8(sp, ls);

    sp = sp.wrapping_sub(1);
    gameboy.mmu.set8(sp, ms);

    gameboy.registers.set16(Registers16::SP, sp);
}

/* Does nothing, pc advances 1
 */
pub fn nop(gameboy: &mut Gameboy) {
    gameboy.advance_cycles(4);
}

/* Increment and Decrements */

/* Incremenet memory pointed to by register r
 */
pub fn inc_ar16(gameboy: &mut Gameboy, r:Registers16) {
    let address = gameboy.registers.get16(r);
    let i = gameboy.mmu.get8(address);
    let (v, overflow) = i.overflowing_add(1);

    gameboy.mmu.set8(address, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry8(i, 1));


    gameboy.advance_cycles(12);
}

/* Decrement memory pointed to by register r
 */
pub fn dec_ar16(gameboy: &mut Gameboy, r:Registers16) {
    let address = gameboy.registers.get16(r);
    let i = gameboy.mmu.get8(address);
    let (v, overflow) = i.overflowing_sub(1);

    gameboy.mmu.set8(address, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(i, 1));


    gameboy.advance_cycles(12);
}

/* Incremenet register r
 */
pub fn inc_r16(gameboy: &mut Gameboy, r:Registers16) {
    let i = gameboy.registers.get16(r);
    let (v, overflow) = i.overflowing_add(1);

    gameboy.registers.set16(r, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry16(i, 1));


    gameboy.advance_cycles(8);
}

/* Decrement register r
 */
pub fn dec_r16(gameboy: &mut Gameboy, r:Registers16) {
    let i = gameboy.registers.get16(r);
    let (v, overflow) = i.overflowing_sub(1);

    gameboy.registers.set16(r, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, true);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry_sub16(i, 1));


    gameboy.advance_cycles(8);
}

/* Increment register r
 */
pub fn inc_r8(gameboy: &mut Gameboy, r:Registers8) {
    let i = gameboy.registers.get8(r);
    let (v, overflow) = i.overflowing_add(1);

    gameboy.registers.set8(r, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry8(i, 1));
    gameboy.advance_cycles(4);
}


/* Decrement register r
 */
pub fn dec_r8(gameboy: &mut Gameboy, r:Registers8) {
    let i = gameboy.registers.get8(r);
    let (v, overflow) = i.overflowing_sub(1);

    gameboy.registers.set8(r, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, true);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(i, 1));

    gameboy.advance_cycles(4);
}

/* Loads */

/* Loads a 8 bit value from r1 into the memory addressed by r2
 */
pub fn ld_ar16_r8(gameboy: &mut Gameboy, r1: Registers16, r2: Registers8) {
    let address = gameboy.registers.get16(r1);
    let value = gameboy.registers.get8(r2);
    gameboy.mmu.set8(address, value);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit immediate value into the memory addressed by r
 */
pub fn ld_ar16_n8(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.fetch_arg_8();
    gameboy.mmu.set8(address, value);
    gameboy.advance_cycles(12)
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 */
pub fn ld_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let address = gameboy.registers.get16(r2);
    let value = gameboy.mmu.get8(address);
    gameboy.registers.set8(r1, value);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit value from the memory addressed by a 16 bit immediate value into r1
 */
pub fn ld_r8_an16(gameboy: &mut Gameboy, r: Registers8) {
    let address = gameboy.fetch_arg_16();
    let value = gameboy.mmu.get8(address);
    gameboy.registers.set8(r, value);
    gameboy.advance_cycles(16)
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 * and simultaneously increments r2
 */
pub fn ldi_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    ld_r8_ar16(gameboy, r1, r2);
    gameboy.registers.inc16(r2);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 * and simultaneously decements r2
 */
pub fn ldd_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    ld_r8_ar16(gameboy, r1, r2);
    gameboy.registers.dec16(r2);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit value from r2 into the memory addressed by r1
 * and simultaneously increments r1
 */
pub fn ldi_ar16_r8(gameboy: &mut Gameboy, r1: Registers16, r2: Registers8) {
    ld_ar16_r8(gameboy, r1, r2);
    gameboy.registers.inc16(Registers16::HL);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit value from r2 into the memory addressed by r1
 * and simultaneously decrements r1
 */
pub fn ldd_ar16_r8(gameboy: &mut Gameboy, r1: Registers16, r2: Registers8) {
    ld_ar16_r8(gameboy, r1, r2);
    gameboy.registers.dec16(Registers16::HL);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit immediate value into r
 */
pub fn ld_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.fetch_arg_8();
    gameboy.registers.set8(r, value);
    gameboy.advance_cycles(8)
}

/* Loads a 8 bit value from r2 into r1
 */
pub fn ld_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let value = gameboy.registers.get8(r2);
    gameboy.registers.set8(r1, value);
    gameboy.advance_cycles(4)
}

/* Loads a 16 bit value from r into the the memory addressed by a 16 bit immediate value
 */
pub fn ld_an16_r16(gameboy: &mut Gameboy, r: Registers16) {
    let value = gameboy.registers.get16(r);
    let address = gameboy.fetch_arg_16();
    gameboy.mmu.set16(address, value);
    gameboy.advance_cycles(20);
}

/* Loads an 8 bit value from r into the the memory addressed by a 16 bit immediate value 
 */
pub fn ld_an16_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let address = gameboy.fetch_arg_16();
    gameboy.mmu.set8(address, value);
    gameboy.advance_cycles(16);
}

/* Loads a 16 bit value from args into the register r
 */
pub fn ld_r16_n16(gameboy: &mut Gameboy, r: Registers16) {
    let value = gameboy.fetch_arg_16();
    gameboy.registers.set16(r, value);
    gameboy.advance_cycles(12);
}

/* Loads a 16 bit value from r1 into r2
 */
pub fn ld_r16_r16(gameboy: &mut Gameboy, r1: Registers16, r2: Registers16) {
    let value = gameboy.registers.get16(r1);
    gameboy.registers.set16(r2, value);
    gameboy.advance_cycles(12);
}

pub fn ld_r16_spn8(gameboy: &mut Gameboy, r: Registers16) {
    let a = gameboy.registers.get16(Registers16::SP);
    let b = gameboy.fetch_arg_8();

    let v = _add_u16_i8(gameboy, a, b);

    gameboy.registers.set16(r, v);


    gameboy.advance_cycles(12)
}

pub fn ldh_an8_r8(gameboy: &mut Gameboy, r: Registers8) {
    let v = gameboy.registers.get8(r);
    let an = gameboy.fetch_arg_8() as u16;
    gameboy.mmu.set8(0xFF00 + an, v);
    gameboy.advance_cycles(12);
}

pub fn ldh_r8_an8(gameboy: &mut Gameboy, r: Registers8) {
    let an = gameboy.fetch_arg_8() as u16;
    let v = gameboy.mmu.get8(0xFF00 + an);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(12);
}

pub fn ldc_ar8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let ar = gameboy.registers.get8(r1) as u16;
    let v = gameboy.registers.get8(r2);

    gameboy.mmu.set8(0xFF00 + ar, v);
    gameboy.advance_cycles(12);
}

pub fn ldc_r8_ar8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let ar = gameboy.registers.get8(r2) as u16;
    let v = gameboy.mmu.get8(0xFF00 + ar);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(12);
}

/* Shifts and Rotates */

/* Rotates the A register left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rla(gameboy: &mut Gameboy) {
    let value = gameboy.registers.get8(Registers8::A);
    let out = _rl(gameboy, value);
    gameboy.registers.set8(Registers8::A, out);
    gameboy.advance_cycles(4);
}

/* Rotates the register r left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rl_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _rl(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(8);
}

/* Rotates memory addressed by register r left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rl_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _rl(gameboy, value);
    gameboy.mmu.set8(address, out);
    gameboy.advance_cycles(16);
}

/* Rotates the A register right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrca(gameboy: &mut Gameboy) {
    let value = gameboy.registers.get8(Registers8::A);
    let out = _rrc(gameboy, value);
    gameboy.registers.set8(Registers8::A, out);
    gameboy.advance_cycles(4);
}

/* Rotates the register r right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrc_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _rrc(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(8);
}

/* Rotates the memory pointed to by r16 right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrc_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _rrc(gameboy, value);
    gameboy.mmu.set8(address, out);
    gameboy.advance_cycles(16);
}

/* Rotates the A register right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rra(gameboy: &mut Gameboy) {
    let value = gameboy.registers.get8(Registers8::A);
    let out = _rr(gameboy, value);
    gameboy.registers.set8(Registers8::A, out);
    gameboy.advance_cycles(4);
}

/* Rotates the register r right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rr_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _rr(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(9);
}

/* Rotates the memory pointed to from r right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rr_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _rr(gameboy, value);
    gameboy.mmu.set8(address, out);
    gameboy.advance_cycles(16);
}

/* Rotates the A register left, puts the shifted bit in c
 * If you have C=1 00010001 and call RLCA the result is C=0 00100010
 * the left most bit is shifted onto C but isn't rotated
 */
pub fn rlca(gameboy: &mut Gameboy) {
    let value = gameboy.registers.get8(Registers8::A);
    let out = _rlc(gameboy, value);
    gameboy.registers.set8(Registers8::A, out);
    gameboy.advance_cycles(4);
}

pub fn rlc_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _rlc(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(8);
}

pub fn rlc_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _rlc(gameboy, value);
    gameboy.mmu.set8(address, out);
    gameboy.advance_cycles(16);
}

/* Shift the contents of register r left into Carry. LSB of n set to 0.
 */
pub fn sla_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _sla(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(8);
}

/* Shift the memory addressed by r left into Carry. LSB of n set to 0.
 */
pub fn sla_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _sla(gameboy, value);
    gameboy.mmu.set8(address, out);
    gameboy.advance_cycles(16);
}

/* Shift the contents of register r right into Carry. LSB of n set to 0.
 */
pub fn sra_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _sra(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(8);
}

/* Shift the memory addressed by r right into Carry. LSB of n set to 0.
 */
pub fn sra_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _sra(gameboy, value);
    gameboy.mmu.set8(address, out);
    gameboy.advance_cycles(16);
}

/* Halt CPU & LCD display until button pressed
 */
pub fn stop(gameboy: &mut Gameboy) {
    gameboy.stop();
    gameboy.advance_cycles(4);
}

/* Halt CPU & LCD display until button pressed
 */
pub fn halt(gameboy: &mut Gameboy) {
    gameboy.halt();
    gameboy.advance_cycles(4);
}

/* Jumps */

pub fn jp_f_n16(gameboy: &mut Gameboy, jump_flag: JumpFlag) {
    let n = gameboy.fetch_arg_16();

    match jump_flag {
        JumpFlag::NZ => {
            if !gameboy.registers.get_flag(Flag::Z) {
                _jump(gameboy, n);
            }
        },
        JumpFlag::Z => {
            if gameboy.registers.get_flag(Flag::Z) {
                _jump(gameboy, n);
            }
        },
        JumpFlag::NC => {
            if !gameboy.registers.get_flag(Flag::C) {
                _jump(gameboy, n);
            }
        }
        JumpFlag::C => {
            if gameboy.registers.get_flag(Flag::C) {
                _jump(gameboy, n);
            }
        }
    }

    gameboy.advance_cycles(12);
}

pub fn jp_n16(gameboy: &mut Gameboy) {
    let n = gameboy.fetch_arg_16();
    _jump(gameboy, n);
    gameboy.advance_cycles(12);
}

pub fn jp_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let n = gameboy.registers.get16(r);
    _jump(gameboy, n);
    gameboy.advance_cycles(4);
}

pub fn call_n16(gameboy: &mut Gameboy) {
    let v = gameboy.fetch_arg_16();
    push_r16(gameboy, Registers16::PC);
    _jump(gameboy, v);
    gameboy.advance_cycles(12);
}

pub fn call_f_n16(gameboy: &mut Gameboy, jump_flag: JumpFlag) {
    let n = gameboy.fetch_arg_16();

    match jump_flag {
        JumpFlag::NZ => {
            if !gameboy.registers.get_flag(Flag::Z) {
                _call(gameboy, n);
            }
        },
        JumpFlag::Z => {
            if gameboy.registers.get_flag(Flag::Z) {
                _call(gameboy, n);
            }
        },
        JumpFlag::NC => {
            if !gameboy.registers.get_flag(Flag::C) {
                _call(gameboy, n);
            }
        }
        JumpFlag::C => {
            if gameboy.registers.get_flag(Flag::C) {
                _call(gameboy, n);
            }
        }
    }

    gameboy.advance_cycles(12);
}

pub fn push_r16(gameboy: &mut Gameboy, r: Registers16) {
    _push(gameboy, r);
    gameboy.advance_cycles(16)
}

pub fn rst_f(gameboy: &mut Gameboy, f: RstFlag) {
    let location = rst_locations(f);
    _push(gameboy, Registers16::PC);
    _jump(gameboy, location as u16);
    gameboy.advance_cycles(32);

}

pub fn di(gameboy: &mut Gameboy) {
    gameboy.disable_interrupts();
    gameboy.advance_cycles(4);
}

pub fn ei(gameboy: &mut Gameboy) {
    gameboy.enable_interrupts();
    gameboy.advance_cycles(4);
}

/* Pop two bytes from stack & jump to that address
 *
 * Note: Jumping is just setting the PC register
 * So we can simplify this function by just passing PC
 * to the _pop function that takes the values from the stack
 * and sets them to the given register.
 */
pub fn ret(gameboy: &mut Gameboy) {
    _ret(gameboy);
    gameboy.advance_cycles(8);
}

pub fn reti(gameboy: &mut Gameboy) {
    _ret(gameboy);
    gameboy.enable_interrupts();
    gameboy.advance_cycles(8);
}

pub fn ret_f(gameboy: &mut Gameboy, jump_flag: JumpFlag) {
    match jump_flag {
        JumpFlag::NZ => {
            if !gameboy.registers.get_flag(Flag::Z) {
                _ret(gameboy);
            }
        },
        JumpFlag::Z => {
            if gameboy.registers.get_flag(Flag::Z) {
                _ret(gameboy);
            }
        },
        JumpFlag::NC => {
            if !gameboy.registers.get_flag(Flag::C) {
                _ret(gameboy);
            }
        }
        JumpFlag::C => {
            if gameboy.registers.get_flag(Flag::C) {
                _ret(gameboy);
            }
        }
    }

    gameboy.advance_cycles(8);
}


pub fn pop_r16(gameboy: &mut Gameboy, r: Registers16) {
    _pop(gameboy, r);
    gameboy.advance_cycles(12);
}

pub fn jr_n8(gameboy: &mut Gameboy) {
    let n = gameboy.fetch_arg_8();
    _jr(gameboy, n);
    gameboy.advance_cycles(8);
}


pub fn jr_f_n8(gameboy: &mut Gameboy, jump_flag: JumpFlag) {
    let n = gameboy.fetch_arg_8();

    match jump_flag {
        JumpFlag::NZ => {
            if !gameboy.registers.get_flag(Flag::Z) {
                _jr(gameboy, n);
            }
        },
        JumpFlag::Z => {
            if gameboy.registers.get_flag(Flag::Z) {
                _jr(gameboy, n);
            }
        },
        JumpFlag::NC => {
            if !gameboy.registers.get_flag(Flag::C) {
                _jr(gameboy, n);
            }
        }
        JumpFlag::C => {
            if gameboy.registers.get_flag(Flag::C) {
                _jr(gameboy, n);
            }
        }
    }

    gameboy.advance_cycles(8);
}

/* Applies the binary complement to the A register
 */
pub fn cpl(gameboy: &mut Gameboy) {
    let v = gameboy.registers.get8(Registers8::A);
    gameboy.registers.set8(Registers8::A, !v);

    gameboy.registers.set_flag(Flag::N, true);
    gameboy.registers.set_flag(Flag::H, true);

    gameboy.advance_cycles(4)
}

/* TODO: Implement */
pub fn daa(gameboy: &mut Gameboy) {
    panic!("DAA not implemented");
}

pub fn scf(gameboy: &mut Gameboy) {

    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, false);

    gameboy.advance_cycles(4)
}

pub fn ccf(gameboy: &mut Gameboy) {
    let c = gameboy.registers.get_flag(Flag::C);

    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, false);
    gameboy.registers.set_flag(Flag::C, !c);

    gameboy.advance_cycles(4)
}


pub fn add_r16_n8(gameboy: &mut Gameboy, r: Registers16) {
    let a = gameboy.registers.get16(r);
    let b = gameboy.fetch_arg_8();


    let v = _add_u16_i8(gameboy, a, b);

    gameboy.registers.set16(r, v);

    gameboy.advance_cycles(16)
}

pub fn add_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _add(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn add_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _add(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn add_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _add(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn adc_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _adc(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn adc_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _adc(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn adc_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _adc(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn add_r16_r16(gameboy: &mut Gameboy, r1: Registers16, r2: Registers16) {
    let a = gameboy.registers.get16(r1);
    let b = gameboy.registers.get16(r2);
    let (v, overflow) = a.overflowing_add(b);

    gameboy.registers.set16(r1, v);

    gameboy.registers.set_flag(Flag::Z, v == 0);
    gameboy.registers.set_flag(Flag::C, overflow);
    gameboy.registers.set_flag(Flag::N, false);
    gameboy.registers.set_flag(Flag::H, bytes::check_half_carry16(a, b));

    gameboy.advance_cycles(8);
}


pub fn sub_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _sub(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn sub_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _sub(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn sub_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _sub(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn sbc_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _sbc(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn sbc_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _sbc(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn sbc_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _sbc(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn and_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _and(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn and_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _and(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn and_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _and(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn xor_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _xor(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn xor_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _xor(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn xor_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _xor(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn or_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    let v = _or(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(4)
}

pub fn or_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    let v = _or(gameboy, a, b);

    gameboy.registers.set8(r, v);
    gameboy.advance_cycles(8)
}

pub fn or_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    let v = _or(gameboy, a, b);

    gameboy.registers.set8(r1, v);
    gameboy.advance_cycles(8)
}

pub fn cp_r8_r8(gameboy: &mut Gameboy, r1: Registers8, r2: Registers8) {
    let a = gameboy.registers.get8(r1);
    let b = gameboy.registers.get8(r2);

    _sub(gameboy, a, b);

    gameboy.advance_cycles(4)
}

pub fn cp_r8_n8(gameboy: &mut Gameboy, r: Registers8) {
    let a = gameboy.registers.get8(r);
    let b = gameboy.fetch_arg_8();

    _sub(gameboy, a, b);

    gameboy.advance_cycles(8)
}

pub fn cp_r8_ar16(gameboy: &mut Gameboy, r1: Registers8, r2: Registers16) {
    let a = gameboy.registers.get8(r1);
    let address = gameboy.registers.get16(r2);
    let b = gameboy.mmu.get8(address);

    _sub(gameboy, a, b);

    gameboy.advance_cycles(8)
}

pub fn swap_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _swap(gameboy, value);
    gameboy.registers.set8(r, out);
    gameboy.advance_cycles(8)
}

pub fn swap_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _swap(gameboy, value);
    gameboy.mmu.set8(address, out);

    gameboy.advance_cycles(16)
}

pub fn srl_r8(gameboy: &mut Gameboy, r: Registers8) {
    let value = gameboy.registers.get8(r);
    let out = _srl(gameboy, value);
    gameboy.registers.set8(r, out);

    gameboy.advance_cycles(8);
}

pub fn srl_ar16(gameboy: &mut Gameboy, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);
    let out = _srl(gameboy, value);
    gameboy.mmu.set8(address, out);

    gameboy.advance_cycles(16);
}

pub fn bit_r8(gameboy: &mut Gameboy, n:u8, r: Registers8) {
    let value = gameboy.registers.get8(r);

    _bit(gameboy, n, value);

    gameboy.advance_cycles(8);
}

pub fn bit_ar16(gameboy: &mut Gameboy, n:u8, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);

    _bit(gameboy, n, value);

    gameboy.advance_cycles(16);
}

pub fn res_r8(gameboy: &mut Gameboy, n:u8, r: Registers8) {
    let value = gameboy.registers.get8(r);

    let out = _res(n, value);

    gameboy.registers.set8(r, out);

    gameboy.advance_cycles(8);
}

pub fn res_ar16(gameboy: &mut Gameboy, n:u8, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);

    let out = _res(n, value);

    gameboy.mmu.set8(address, out);

    gameboy.advance_cycles(16);
}

pub fn set_r8(gameboy: &mut Gameboy, n:u8, r: Registers8) {
    let value = gameboy.registers.get8(r);

    let out = _set(n, value);

    gameboy.registers.set8(r, out);

    gameboy.advance_cycles(8);
}

pub fn set_ar16(gameboy: &mut Gameboy, n:u8, r: Registers16) {
    let address = gameboy.registers.get16(r);
    let value = gameboy.mmu.get8(address);

    let out = _set(n, value);

    gameboy.mmu.set8(address, out);

    gameboy.advance_cycles(16);
}

pub fn illegal_opcode(opcode: &str) {
    panic!("attempted to call: {}", opcode);
}
