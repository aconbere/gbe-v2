use super::gameboy::Gameboy;
use super::register::{Registers16, Registers8, Flag};
use super::bytes;

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


pub fn execute(mut gameboy: &mut Gameboy, opcode: u16) {
    match opcode {
        0x0000 => nop(&mut gameboy),
        0x0001 => ld_r16_n16(&mut gameboy, Registers16::BC),
        0x0002 => ld_ar16_r8(&mut gameboy, Registers16::BC, Registers8::A),
        0x0003 => inc_r16(&mut gameboy, Registers16::BC),
        0x0004 => inc_r8(&mut gameboy, Registers8::B),
        0x0005 => dec_r8(&mut gameboy, Registers8::B),
        0x0006 => ld_r8_n8(&mut gameboy, Registers8::B),
        0x0007 => rlca(&mut gameboy),
        0x0008 => ld_an16_r16(&mut gameboy, Registers16::SP),
        0x0009 => add_r16_r16(&mut gameboy, Registers16::HL, Registers16::BC),
        0x000A => ld_r8_ar16(&mut gameboy, Registers8::A, Registers16::BC),
        0x000B => dec_r16(&mut gameboy, Registers16::BC),
        0x000C => inc_r8(&mut gameboy, Registers8::C),
        0x000D => dec_r8(&mut gameboy, Registers8::C),
        0x000E => ld_r8_n8(&mut gameboy, Registers8::C),
        0x000F => rrca(&mut gameboy),

        0x0010 => stop(&mut gameboy),
        0x0011 => ld_r16_n16(&mut gameboy, Registers16::DE),
        0x0012 => ld_ar16_r8(&mut gameboy, Registers16::DE, Registers8::A),
        0x0013 => inc_r16(&mut gameboy, Registers16::DE),
        0x0014 => inc_r8(&mut gameboy, Registers8::D),
        0x0015 => dec_r8(&mut gameboy, Registers8::D),
        0x0016 => ld_r8_n8(&mut gameboy, Registers8::D),
        0x0017 => rla(&mut gameboy),
        0x0018 => jr_n8(&mut gameboy),
        0x0019 => add_r16_r16(&mut gameboy, Registers16::HL, Registers16::DE),
        0x001A => ld_r8_ar16(&mut gameboy, Registers8::A, Registers16::DE),
        0x001B => dec_r16(&mut gameboy, Registers16::DE),
        0x001C => inc_r8(&mut gameboy, Registers8::E),
        0x001D => dec_r8(&mut gameboy, Registers8::E),
        0x001E => ld_r8_n8(&mut gameboy, Registers8::E),
        0x001F => rra(&mut gameboy),

        0x0020 => jr_f_n8(&mut gameboy, JumpFlag::NZ),
        0x0021 => ld_r16_n16(&mut gameboy, Registers16::HL),
        0x0022 => ldi_ar16_r8(&mut gameboy, Registers16::HL, Registers8::A),
        0x0023 => inc_r16(&mut gameboy, Registers16::HL),
        0x0024 => inc_r8(&mut gameboy, Registers8::H),
        0x0025 => dec_r8(&mut gameboy, Registers8::H),
        0x0026 => ld_r8_n8(&mut gameboy, Registers8::H),
        0x0027 => daa(&mut gameboy),
        0x0028 => jr_f_n8(&mut gameboy, JumpFlag::Z),
        0x0029 => add_r16_r16(&mut gameboy, Registers16::HL, Registers16::HL),
        0x002A => ldi_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL),
        0x002B => dec_r16(&mut gameboy, Registers16::HL),
        0x002C => inc_r8(&mut gameboy, Registers8::L),
        0x002D => dec_r8(&mut gameboy, Registers8::L),
        0x002E => ld_r8_n8(&mut gameboy, Registers8::L),
        0x002F => cpl(&mut gameboy),

        0x0030 => jr_f_n8(&mut gameboy, JumpFlag::NC),
        0x0031 => ld_r16_n16(&mut gameboy, Registers16::SP),
        0x0032 => ldd_ar16_r8(&mut gameboy, Registers16::HL, Registers8::A),
        0x0033 => inc_r16(&mut gameboy, Registers16::SP),
        0x0034 => inc_ar16(&mut gameboy, Registers16::HL),
        0x0035 => dec_ar16(&mut gameboy, Registers16::HL),
        0x0036 => ld_ar16_n8(&mut gameboy, Registers16::HL),
        0x0037 => scf(&mut gameboy),
        0x0038 => jr_f_n8(&mut gameboy, JumpFlag::C),
        0x0039 => add_r16_r16(&mut gameboy, Registers16::HL, Registers16::SP),
        0x003A => ldd_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL),
        0x003B => dec_r16(&mut gameboy, Registers16::SP),
        0x003C => inc_r8(&mut gameboy, Registers8::A),
        0x003D => dec_r8(&mut gameboy, Registers8::A),
        0x003E => ld_r8_n8(&mut gameboy, Registers8::A),
        0x003F => ccf(&mut gameboy),

        0x0040 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::B),
        0x0041 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::C),
        0x0042 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::D),
        0x0043 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::E),
        0x0044 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::H),
        0x0045 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::L),
        0x0046 => ld_r8_ar16(&mut gameboy, Registers8::B, Registers16::HL),
        0x0047 => ld_r8_r8(&mut gameboy, Registers8::B, Registers8::A),

        0x0048 => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::B),
        0x0049 => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::C),
        0x004A => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::D),
        0x004B => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::E),
        0x004C => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::H),
        0x004D => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::L),
        0x004E => ld_r8_ar16(&mut gameboy, Registers8::C, Registers16::HL),
        0x004F => ld_r8_r8(&mut gameboy, Registers8::C, Registers8::A),

        0x0050 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::B),
        0x0051 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::C),
        0x0052 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::D),
        0x0053 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::E),
        0x0054 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::H),
        0x0055 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::L),
        0x0056 => ld_r8_ar16(&mut gameboy, Registers8::D, Registers16::HL),
        0x0057 => ld_r8_r8(&mut gameboy, Registers8::D, Registers8::A),

        0x0058 => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::B),
        0x0059 => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::C),
        0x005A => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::D),
        0x005B => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::E),
        0x005C => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::H),
        0x005D => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::L),
        0x005E => ld_r8_ar16(&mut gameboy, Registers8::E, Registers16::HL),
        0x005F => ld_r8_r8(&mut gameboy, Registers8::E, Registers8::A),

        0x0060 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::B),
        0x0061 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::C),
        0x0062 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::D),
        0x0063 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::E),
        0x0064 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::H),
        0x0065 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::L),
        0x0066 => ld_r8_ar16(&mut gameboy, Registers8::H, Registers16::HL),
        0x0067 => ld_r8_r8(&mut gameboy, Registers8::H, Registers8::A),

        0x0068 => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::B),
        0x0069 => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::C),
        0x006A => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::D),
        0x006B => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::E),
        0x006C => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::H),
        0x006D => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::L),
        0x006E => ld_r8_ar16(&mut gameboy, Registers8::L, Registers16::HL),
        0x006F => ld_r8_r8(&mut gameboy, Registers8::L, Registers8::A),

        0x0070 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::B),
        0x0071 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::C),
        0x0072 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::D),
        0x0073 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::E),
        0x0074 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::H),
        0x0075 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::L),
        0x0076 => halt(&mut gameboy),
        0x0077 => ld_ar16_r8(&mut gameboy, Registers16::HL, Registers8::A),

        0x0078 => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::B),
        0x0079 => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::C),
        0x007A => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::D),
        0x007B => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::E),
        0x007C => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::H),
        0x007D => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::L),
        0x007E => ld_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL),
        0x007F => ld_r8_r8(&mut gameboy, Registers8::A, Registers8::A),

        0x0080 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x0081 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x0082 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x0083 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x0084 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x0085 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x0086 => add_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x0087 => add_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x0088 => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x0089 => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x008A => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x008B => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x008C => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x008D => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x008E => adc_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x008F => adc_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x0090 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x0091 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x0092 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x0093 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x0094 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x0095 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x0096 => sub_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x0097 => sub_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x0098 => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x0099 => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x009A => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x009B => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x009C => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x009D => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x009E => sbc_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x009F => sbc_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x00A0 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x00A1 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x00A2 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x00A3 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x00A4 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x00A5 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x00A6 => and_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x00A7 => and_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x00A8 => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x00A9 => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x00AA => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x00AB => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x00AC => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x00AD => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x00AE => xor_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x00AF => xor_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x00B0 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x00B1 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x00B2 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x00B3 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x00B4 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x00B5 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x00B6 => or_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x00B7 => or_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x00B8 => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::B), 
        0x00B9 => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::C), 
        0x00BA => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::D), 
        0x00BB => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::E), 
        0x00BC => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::H), 
        0x00BD => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::L), 
        0x00BE => cp_r8_ar16(&mut gameboy, Registers8::A, Registers16::HL), 
        0x00BF => cp_r8_r8(&mut gameboy, Registers8::A, Registers8::A), 

        0x00C0 => ret_f(&mut gameboy, JumpFlag::NZ),
        0x00C1 => pop_r16(&mut gameboy, Registers16::BC),
        0x00C2 => jp_f_n16(&mut gameboy, JumpFlag::NZ),
        0x00C3 => jp_n16(&mut gameboy),
        0x00C4 => call_f_n16(&mut gameboy, JumpFlag::NZ),
        0x00C5 => push_r16(&mut gameboy, Registers16::BC),
        0x00C6 => add_r8_n8(&mut gameboy, Registers8::A),
        0x00C7 => rst_f(&mut gameboy, RstFlag::H00),

        0x00C8 => ret_f(&mut gameboy, JumpFlag::Z),
        0x00C9 => ret(&mut gameboy),
        0x00CA => jp_f_n16(&mut gameboy, JumpFlag::Z),
        0x00CB => { panic!("attempted to call CB"); },
        0x00CC => call_f_n16(&mut gameboy, JumpFlag::Z),
        0x00CD => call_n16(&mut gameboy),
        0x00CE => adc_r8_n8(&mut gameboy, Registers8::A),
        0x00CF => rst_f(&mut gameboy, RstFlag::H08),

        0x00D0 => ret_f(&mut gameboy, JumpFlag::NC),
        0x00D1 => pop_r16(&mut gameboy, Registers16::DE),
        0x00D2 => jp_f_n16(&mut gameboy, JumpFlag::NC),
        0x00D3 => { panic!("attempted to call D3"); }
        0x00D4 => call_f_n16(&mut gameboy, JumpFlag::NC),
        0x00D5 => push_r16(&mut gameboy, Registers16::DE),
        0x00D6 => sub_r8_n8(&mut gameboy, Registers8::A),
        0x00D7 => rst_f(&mut gameboy, RstFlag::H10),

        0x00D8 => ret_f(&mut gameboy, JumpFlag::C),
        0x00D9 => reti(&mut gameboy),
        0x00DA => jp_f_n16(&mut gameboy, JumpFlag::C),
        0x00DB => { panic!("attempted to call CB"); },
        0x00DC => call_f_n16(&mut gameboy, JumpFlag::C),
        0x00DD => { panic!("attempted to call DD"); },
        0x00DE => sbc_r8_n8(&mut gameboy, Registers8::A),
        0x00DF => rst_f(&mut gameboy, RstFlag::H18),

        0x00E0 => ldh_an8_r8(&mut gameboy, Registers8::A),
        0x00E1 => pop_r16(&mut gameboy, Registers16::HL),
        0x00E2 => ldc_ar8_r8(&mut gameboy, Registers8::C, Registers8::A),
        0x00E3 => { panic!("attempted to call E3"); },
        0x00E4 => { panic!("attempted to call E4"); },
        0x00E5 => push_r16(&mut gameboy, Registers16::HL),
        0x00E6 => and_r8_n8(&mut gameboy, Registers8::A),
        0x00E7 => rst_f(&mut gameboy, RstFlag::H20),

        0x00E8 => add_r16_n8(&mut gameboy, Registers16::SP),
        0x00E9 => jp_ar16(&mut gameboy, Registers16::HL),
        0x00EA => ld_an16_r8(&mut gameboy, Registers8::A),
        0x00EB => { panic!("attempted to call EB"); },
        0x00EC => { panic!("attempted to call EC"); },
        0x00ED => { panic!("attempted to call ED"); },
        0x00EE => xor_r8_n8(&mut gameboy, Registers8::A),
        0x00EF => rst_f(&mut gameboy, RstFlag::H28),

        0x00F0 => ldh_r8_an8(&mut gameboy, Registers8::A),
        0x00F1 => pop_r16(&mut gameboy, Registers16::AF),
        0x00F2 => ldc_r8_ar8(&mut gameboy, Registers8::A, Registers8::C),
        0x00F3 => di(&mut gameboy),
        0x00F4 => { panic!("attempted to call F4"); },
        0x00F5 => push_r16(&mut gameboy, Registers16::AF),
        0x00F6 => or_r8_n8(&mut gameboy, Registers8::A),
        0x00F7 => rst_f(&mut gameboy, RstFlag::H30),

        0x00F8 => ld_r16_spn8(&mut gameboy, Registers16::SP),
        0x00F9 => ld_r16_r16(&mut gameboy, Registers16::SP, Registers16::HL),
        0x00FA => ld_r8_an16(&mut gameboy, Registers8::A),
        0x00FB => ei(&mut gameboy),
        0x00FC => { panic!("attempted to call FC"); },
        0x00FD => { panic!("attempted to call FD"); },
        0x00FE => cp_r8_n8(&mut gameboy, Registers8::A),
        0x00FF => rst_f(&mut gameboy, RstFlag::H38),

        // Prefix CB

        0x0100 => rlc_r8(gameboy, Registers8::B),
        0x0101 => rlc_r8(gameboy, Registers8::C),
        0x0102 => rlc_r8(gameboy, Registers8::D),
        0x0103 => rlc_r8(gameboy, Registers8::E),
        0x0104 => rlc_r8(gameboy, Registers8::H),
        0x0105 => rlc_r8(gameboy, Registers8::L),
        0x0106 => rlc_ar16(gameboy, Registers16::HL),
        0x0107 => rlc_r8(gameboy, Registers8::A),

        0x0108 => rrc_r8(gameboy, Registers8::B),
        0x0109 => rrc_r8(gameboy, Registers8::C),
        0x010A => rrc_r8(gameboy, Registers8::D),
        0x010B => rrc_r8(gameboy, Registers8::E),
        0x010C => rrc_r8(gameboy, Registers8::H),
        0x010D => rrc_r8(gameboy, Registers8::L),
        0x010E => rrc_ar16(gameboy, Registers16::HL),
        0x010F => rrc_r8(gameboy, Registers8::A),

        0x0110 => rl_r8(gameboy, Registers8::B),
        0x0111 => rl_r8(gameboy, Registers8::C),
        0x0112 => rl_r8(gameboy, Registers8::D),
        0x0113 => rl_r8(gameboy, Registers8::E),
        0x0114 => rl_r8(gameboy, Registers8::H),
        0x0115 => rl_r8(gameboy, Registers8::L),
        0x0116 => rl_ar16(gameboy, Registers16::HL),
        0x0117 => rl_r8(gameboy, Registers8::A),

        0x0118 => rr_r8(gameboy, Registers8::B),
        0x0119 => rr_r8(gameboy, Registers8::C),
        0x011A => rr_r8(gameboy, Registers8::D),
        0x011B => rr_r8(gameboy, Registers8::E),
        0x011C => rr_r8(gameboy, Registers8::H),
        0x011D => rr_r8(gameboy, Registers8::L),
        0x011E => rr_ar16(gameboy, Registers16::HL),
        0x011F => rr_r8(gameboy, Registers8::A),

        0x0120 => sla_r8(gameboy, Registers8::B),
        0x0121 => sla_r8(gameboy, Registers8::C),
        0x0122 => sla_r8(gameboy, Registers8::D),
        0x0123 => sla_r8(gameboy, Registers8::E),
        0x0124 => sla_r8(gameboy, Registers8::H),
        0x0125 => sla_r8(gameboy, Registers8::L),
        0x0126 => sla_ar16(gameboy, Registers16::HL),
        0x0127 => sla_r8(gameboy, Registers8::A),

        0x0128 => sra_r8(gameboy, Registers8::B),
        0x0129 => sra_r8(gameboy, Registers8::C),
        0x012A => sra_r8(gameboy, Registers8::D),
        0x012B => sra_r8(gameboy, Registers8::E),
        0x012C => sra_r8(gameboy, Registers8::H),
        0x012D => sra_r8(gameboy, Registers8::L),
        0x012E => sra_ar16(gameboy, Registers16::HL),
        0x012F => sra_r8(gameboy, Registers8::A),

        0x0130 => swap_r8(gameboy, Registers8::B),
        0x0131 => swap_r8(gameboy, Registers8::C),
        0x0132 => swap_r8(gameboy, Registers8::D),
        0x0133 => swap_r8(gameboy, Registers8::E),
        0x0134 => swap_r8(gameboy, Registers8::H),
        0x0135 => swap_r8(gameboy, Registers8::L),
        0x0136 => swap_ar16(gameboy, Registers16::HL),
        0x0137 => swap_r8(gameboy, Registers8::A),

        0x0138 => srl_r8(gameboy, Registers8::B),
        0x0139 => srl_r8(gameboy, Registers8::C),
        0x013A => srl_r8(gameboy, Registers8::D),
        0x013B => srl_r8(gameboy, Registers8::E),
        0x013C => srl_r8(gameboy, Registers8::H),
        0x013D => srl_r8(gameboy, Registers8::L),
        0x013E => srl_ar16(gameboy, Registers16::HL),
        0x013F => srl_r8(gameboy, Registers8::A),

        0x0140 => bit_r8(gameboy, 0, Registers8::B),
        0x0141 => bit_r8(gameboy, 0, Registers8::C),
        0x0142 => bit_r8(gameboy, 0, Registers8::D),
        0x0143 => bit_r8(gameboy, 0, Registers8::E),
        0x0144 => bit_r8(gameboy, 0, Registers8::H),
        0x0145 => bit_r8(gameboy, 0, Registers8::L),
        0x0146 => bit_ar16(gameboy, 0, Registers16::HL),
        0x0147 => bit_r8(gameboy, 0, Registers8::A),

        0x0148 => bit_r8(gameboy, 1, Registers8::B),
        0x0149 => bit_r8(gameboy, 1, Registers8::C),
        0x014A => bit_r8(gameboy, 1, Registers8::D),
        0x014B => bit_r8(gameboy, 1, Registers8::E),
        0x014C => bit_r8(gameboy, 1, Registers8::H),
        0x014D => bit_r8(gameboy, 1, Registers8::L),
        0x014E => bit_ar16(gameboy, 1, Registers16::HL),
        0x014F => bit_r8(gameboy, 1, Registers8::A),

        0x0150 => bit_r8(gameboy, 2, Registers8::B),
        0x0151 => bit_r8(gameboy, 2, Registers8::C),
        0x0152 => bit_r8(gameboy, 2, Registers8::D),
        0x0153 => bit_r8(gameboy, 2, Registers8::E),
        0x0154 => bit_r8(gameboy, 2, Registers8::H),
        0x0155 => bit_r8(gameboy, 2, Registers8::L),
        0x0156 => bit_ar16(gameboy, 2, Registers16::HL),
        0x0157 => bit_r8(gameboy, 2, Registers8::A),

        0x0158 => bit_r8(gameboy, 3, Registers8::B),
        0x0159 => bit_r8(gameboy, 3, Registers8::C),
        0x015A => bit_r8(gameboy, 3, Registers8::D),
        0x015B => bit_r8(gameboy, 3, Registers8::E),
        0x015C => bit_r8(gameboy, 3, Registers8::H),
        0x015D => bit_r8(gameboy, 3, Registers8::L),
        0x015E => bit_ar16(gameboy, 3, Registers16::HL),
        0x015F => bit_r8(gameboy, 3, Registers8::A),

        0x0160 => bit_r8(gameboy, 4, Registers8::B),
        0x0161 => bit_r8(gameboy, 4, Registers8::C),
        0x0162 => bit_r8(gameboy, 4, Registers8::D),
        0x0163 => bit_r8(gameboy, 4, Registers8::E),
        0x0164 => bit_r8(gameboy, 4, Registers8::H),
        0x0165 => bit_r8(gameboy, 4, Registers8::L),
        0x0166 => bit_ar16(gameboy, 4, Registers16::HL),
        0x0167 => bit_r8(gameboy, 4, Registers8::A),

        0x0168 => bit_r8(gameboy, 5, Registers8::B),
        0x0169 => bit_r8(gameboy, 5, Registers8::C),
        0x016A => bit_r8(gameboy, 5, Registers8::D),
        0x016B => bit_r8(gameboy, 5, Registers8::E),
        0x016C => bit_r8(gameboy, 5, Registers8::H),
        0x016D => bit_r8(gameboy, 5, Registers8::L),
        0x016E => bit_ar16(gameboy, 5, Registers16::HL),
        0x016F => bit_r8(gameboy, 5, Registers8::A),

        0x0170 => bit_r8(gameboy, 6, Registers8::B),
        0x0171 => bit_r8(gameboy, 6, Registers8::C),
        0x0172 => bit_r8(gameboy, 6, Registers8::D),
        0x0173 => bit_r8(gameboy, 6, Registers8::E),
        0x0174 => bit_r8(gameboy, 6, Registers8::H),
        0x0175 => bit_r8(gameboy, 6, Registers8::L),
        0x0176 => bit_ar16(gameboy, 6, Registers16::HL),
        0x0177 => bit_r8(gameboy, 6, Registers8::A),

        0x0178 => bit_r8(gameboy, 7, Registers8::B),
        0x0179 => bit_r8(gameboy, 7, Registers8::C),
        0x017A => bit_r8(gameboy, 7, Registers8::D),
        0x017B => bit_r8(gameboy, 7, Registers8::E),
        0x017C => bit_r8(gameboy, 7, Registers8::H),
        0x017D => bit_r8(gameboy, 7, Registers8::L),
        0x017E => bit_ar16(gameboy, 7, Registers16::HL),
        0x017F => bit_r8(gameboy, 7, Registers8::A),

        0x0180 => res_r8(gameboy, 0, Registers8::B),
        0x0181 => res_r8(gameboy, 0, Registers8::C),
        0x0182 => res_r8(gameboy, 0, Registers8::D),
        0x0183 => res_r8(gameboy, 0, Registers8::E),
        0x0184 => res_r8(gameboy, 0, Registers8::H),
        0x0185 => res_r8(gameboy, 0, Registers8::L),
        0x0186 => res_ar16(gameboy, 0, Registers16::HL),
        0x0187 => res_r8(gameboy, 0, Registers8::A),

        0x0188 => res_r8(gameboy, 1, Registers8::B),
        0x0189 => res_r8(gameboy, 1, Registers8::C),
        0x018A => res_r8(gameboy, 1, Registers8::D),
        0x018B => res_r8(gameboy, 1, Registers8::E),
        0x018C => res_r8(gameboy, 1, Registers8::H),
        0x018D => res_r8(gameboy, 1, Registers8::L),
        0x018E => res_ar16(gameboy, 1, Registers16::HL),
        0x018F => res_r8(gameboy, 1, Registers8::A),

        0x0190 => res_r8(gameboy, 2, Registers8::B),
        0x0191 => res_r8(gameboy, 2, Registers8::C),
        0x0192 => res_r8(gameboy, 2, Registers8::D),
        0x0193 => res_r8(gameboy, 2, Registers8::E),
        0x0194 => res_r8(gameboy, 2, Registers8::H),
        0x0195 => res_r8(gameboy, 2, Registers8::L),
        0x0196 => res_ar16(gameboy, 2, Registers16::HL),
        0x0197 => res_r8(gameboy, 2, Registers8::A),

        0x0198 => res_r8(gameboy, 3, Registers8::B),
        0x0199 => res_r8(gameboy, 3, Registers8::C),
        0x019A => res_r8(gameboy, 3, Registers8::D),
        0x019B => res_r8(gameboy, 3, Registers8::E),
        0x019C => res_r8(gameboy, 3, Registers8::H),
        0x019D => res_r8(gameboy, 3, Registers8::L),
        0x019E => res_ar16(gameboy, 3, Registers16::HL),
        0x019F => res_r8(gameboy, 3, Registers8::A),

        0x01A0 => res_r8(gameboy, 4, Registers8::B),
        0x01A1 => res_r8(gameboy, 4, Registers8::C),
        0x01A2 => res_r8(gameboy, 4, Registers8::D),
        0x01A3 => res_r8(gameboy, 4, Registers8::E),
        0x01A4 => res_r8(gameboy, 4, Registers8::H),
        0x01A5 => res_r8(gameboy, 4, Registers8::L),
        0x01A6 => res_ar16(gameboy, 4, Registers16::HL),
        0x01A7 => res_r8(gameboy, 4, Registers8::A),

        0x01A8 => res_r8(gameboy, 5, Registers8::B),
        0x01A9 => res_r8(gameboy, 5, Registers8::C),
        0x01AA => res_r8(gameboy, 5, Registers8::D),
        0x01AB => res_r8(gameboy, 5, Registers8::E),
        0x01AC => res_r8(gameboy, 5, Registers8::H),
        0x01AD => res_r8(gameboy, 5, Registers8::L),
        0x01AE => res_ar16(gameboy, 5, Registers16::HL),
        0x01AF => res_r8(gameboy, 5, Registers8::A),

        0x01B0 => res_r8(gameboy, 6, Registers8::B),
        0x01B1 => res_r8(gameboy, 6, Registers8::C),
        0x01B2 => res_r8(gameboy, 6, Registers8::D),
        0x01B3 => res_r8(gameboy, 6, Registers8::E),
        0x01B4 => res_r8(gameboy, 6, Registers8::H),
        0x01B5 => res_r8(gameboy, 6, Registers8::L),
        0x01B6 => res_ar16(gameboy, 6, Registers16::HL),
        0x01B7 => res_r8(gameboy, 6, Registers8::A),

        0x01B8 => res_r8(gameboy, 7, Registers8::B),
        0x01B9 => res_r8(gameboy, 7, Registers8::C),
        0x01BA => res_r8(gameboy, 7, Registers8::D),
        0x01BB => res_r8(gameboy, 7, Registers8::E),
        0x01BC => res_r8(gameboy, 7, Registers8::H),
        0x01BD => res_r8(gameboy, 7, Registers8::L),
        0x01BE => res_ar16(gameboy, 7, Registers16::HL),
        0x01BF => res_r8(gameboy, 7, Registers8::A),

        0x01C0 => set_r8(gameboy, 0, Registers8::B),
        0x01C1 => set_r8(gameboy, 0, Registers8::C),
        0x01C2 => set_r8(gameboy, 0, Registers8::D),
        0x01C3 => set_r8(gameboy, 0, Registers8::E),
        0x01C4 => set_r8(gameboy, 0, Registers8::H),
        0x01C5 => set_r8(gameboy, 0, Registers8::L),
        0x01C6 => set_ar16(gameboy, 0, Registers16::HL),
        0x01C7 => set_r8(gameboy, 0, Registers8::A),

        0x01C8 => set_r8(gameboy, 1, Registers8::B),
        0x01C9 => set_r8(gameboy, 1, Registers8::C),
        0x01CA => set_r8(gameboy, 1, Registers8::D),
        0x01CB => set_r8(gameboy, 1, Registers8::E),
        0x01CC => set_r8(gameboy, 1, Registers8::H),
        0x01CD => set_r8(gameboy, 1, Registers8::L),
        0x01CE => set_ar16(gameboy, 1, Registers16::HL),
        0x01CF => set_r8(gameboy, 1, Registers8::A),

        0x01D0 => set_r8(gameboy, 2, Registers8::B),
        0x01D1 => set_r8(gameboy, 2, Registers8::C),
        0x01D2 => set_r8(gameboy, 2, Registers8::D),
        0x01D3 => set_r8(gameboy, 2, Registers8::E),
        0x01D4 => set_r8(gameboy, 2, Registers8::H),
        0x01D5 => set_r8(gameboy, 2, Registers8::L),
        0x01D6 => set_ar16(gameboy, 2, Registers16::HL),
        0x01D7 => set_r8(gameboy, 2, Registers8::A),

        0x01D8 => set_r8(gameboy, 3, Registers8::B),
        0x01D9 => set_r8(gameboy, 3, Registers8::C),
        0x01DA => set_r8(gameboy, 3, Registers8::D),
        0x01DB => set_r8(gameboy, 3, Registers8::E),
        0x01DC => set_r8(gameboy, 3, Registers8::H),
        0x01DD => set_r8(gameboy, 3, Registers8::L),
        0x01DE => set_ar16(gameboy, 3, Registers16::HL),
        0x01DF => set_r8(gameboy, 3, Registers8::A),

        0x01E0 => set_r8(gameboy, 4, Registers8::B),
        0x01E1 => set_r8(gameboy, 4, Registers8::C),
        0x01E2 => set_r8(gameboy, 4, Registers8::D),
        0x01E3 => set_r8(gameboy, 4, Registers8::E),
        0x01E4 => set_r8(gameboy, 4, Registers8::H),
        0x01E5 => set_r8(gameboy, 4, Registers8::L),
        0x01E6 => set_ar16(gameboy, 4, Registers16::HL),
        0x01E7 => set_r8(gameboy, 4, Registers8::A),

        0x01E8 => set_r8(gameboy, 5, Registers8::B),
        0x01E9 => set_r8(gameboy, 5, Registers8::C),
        0x01EA => set_r8(gameboy, 5, Registers8::D),
        0x01EB => set_r8(gameboy, 5, Registers8::E),
        0x01EC => set_r8(gameboy, 5, Registers8::H),
        0x01ED => set_r8(gameboy, 5, Registers8::L),
        0x01EE => set_ar16(gameboy, 5, Registers16::HL),
        0x01EF => set_r8(gameboy, 5, Registers8::A),

        0x01F0 => set_r8(gameboy, 6, Registers8::B),
        0x01F1 => set_r8(gameboy, 6, Registers8::C),
        0x01F2 => set_r8(gameboy, 6, Registers8::D),
        0x01F3 => set_r8(gameboy, 6, Registers8::E),
        0x01F4 => set_r8(gameboy, 6, Registers8::H),
        0x01F5 => set_r8(gameboy, 6, Registers8::L),
        0x01F6 => set_ar16(gameboy, 6, Registers16::HL),
        0x01F7 => set_r8(gameboy, 6, Registers8::A),

        0x01F8 => set_r8(gameboy, 7, Registers8::B),
        0x01F9 => set_r8(gameboy, 7, Registers8::C),
        0x01FA => set_r8(gameboy, 7, Registers8::D),
        0x01FB => set_r8(gameboy, 7, Registers8::E),
        0x01FC => set_r8(gameboy, 7, Registers8::H),
        0x01FD => set_r8(gameboy, 7, Registers8::L),
        0x01FE => set_ar16(gameboy, 7, Registers16::HL),
        0x01FF => set_r8(gameboy, 7, Registers8::A),

        _ => panic!("not implemented"),
    }
}
