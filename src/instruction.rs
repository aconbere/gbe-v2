use crate::cpu::CPU;
use crate::register::{Registers8, Registers16, Flag, IME};
use crate::bytes;

pub mod helper;
pub mod opcode;

pub struct OpResult {
    pub cycles: u8,
}

pub struct Instruction {
    pub f: IFn,
    pub description: String,
    pub args: u8,
}

impl Instruction {
    pub fn new(description: String, args: u8, f: IFn) -> Instruction {
        Instruction {
            description: description.to_string(),
            args: args,
            f: f,
        }
    }

    pub fn no_args(description: String, f: IFn) -> Instruction {
        Instruction::new(description.to_string(), 0, f)
    }

    pub fn arg8(description: String, f: IFn) -> Instruction {
        Instruction::new(description.to_string(), 1, f)
    }

    pub fn arg16(description: String, f: IFn) -> Instruction {
        Instruction::new(description.to_string(), 2, f)
    }

    pub fn call(&self, cpu: &mut CPU, arg: u16) -> OpResult {
        (self.f)(cpu, arg)
    }
}

pub type IFn = Box<dyn Fn(&mut CPU, u16) -> OpResult>;

#[derive(Debug, Clone, Copy)]
pub enum RstFlag {
    H00, H08, H10, H18, H20, H28, H30, H38
}

fn rst_locations(f: RstFlag) -> u16 {
    match f {
        RstFlag::H00 => 0x0000,
        RstFlag::H08 => 0x0008,
        RstFlag::H10 => 0x0010,
        RstFlag::H18 => 0x0018,
        RstFlag::H20 => 0x0020,
        RstFlag::H28 => 0x0028,
        RstFlag::H30 => 0x0030,
        RstFlag::H38 => 0x0038,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JumpFlag {
    NZ, Z, NC, C
}



fn cycles(a: u8) -> OpResult {
    OpResult {
        cycles: a,
    }
}

/* Does nothing, pc advances 1
 */
pub fn nop() -> Instruction {
    Instruction::no_args(
        String::from("NOP"), 
        Box::new(move |_cpu: &mut CPU, _arg: u16| {
            cycles(4)
        }))
}


/* Increment and Decrements */

/* Decrement register r
 */
pub fn dec_r8(r:Registers8) -> Instruction {
    Instruction::no_args(
        format!("DEC R8: {:?}", r), 
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let i = cpu.registers.get8(r);

            let v = i.wrapping_sub(1);

            cpu.registers.set8(r, v);

            cpu.registers.set_flag(Flag::Z, v == 0);
            cpu.registers.set_flag(Flag::N, true);
            cpu.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(i, 1));
            cycles(4)
        }))
}


/* Decrement register r
 */
pub fn dec_r16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("DEC R16: {:?}", r), 
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let i = cpu.registers.get16(r);

            let v = i.wrapping_sub(1);

            cpu.registers.set16(r, v);

            cycles(8)
        }))
}

/* Decrement memory pointed to by register r
 */
pub fn dec_ar16(r:Registers16) -> Instruction {
    Instruction::no_args(
        format!("DEC AR16: {:?}", r), 
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let i = cpu.mmu.get(address);

            let v = i.wrapping_sub(1);

            cpu.mmu.set(address, v);

            cpu.registers.set_flag(Flag::Z, v == 0);
            cpu.registers.set_flag(Flag::N, true);
            cpu.registers.set_flag(Flag::H, bytes::check_half_carry_sub8(i, 1));


            cycles(12)
        }))
}

/* Increment register r
 */
pub fn inc_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("INC R8: {:?}", r), 
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let i = cpu.registers.get8(r);
            let v = i.wrapping_add(1);

            cpu.registers.set8(r, v);

            cpu.registers.set_flag(Flag::Z, v == 0);
            cpu.registers.set_flag(Flag::N, false);
            cpu.registers.set_flag(Flag::H, bytes::check_half_carry8(i, 1));

            cycles(4)
        }))
}

/* Incremenet register r
 */
pub fn inc_r16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("INC R16: {:?}", r), 
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let i = cpu.registers.get16(r);
            let v = i.wrapping_add(1);

            cpu.registers.set16(r, v);

            cycles(8)
        }))
}



/* Incremenet memory pointed to by register r
 */
pub fn inc_ar16(r:Registers16) -> Instruction {
    Instruction::no_args(
        format!("INC AR16: {:?}", r), 
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let i = cpu.mmu.get(address);
            let v = i.wrapping_add(1);

            cpu.mmu.set(address, v);

            cpu.registers.set_flag(Flag::Z, v == 0);
            cpu.registers.set_flag(Flag::N, false);
            cpu.registers.set_flag(Flag::H, bytes::check_half_carry8(i, 1));


            cycles(12)
        }))
}


/* Loads */

/* Loads a 8 bit value from r2 into the memory addressed by r1
 */
pub fn ld_ar16_r8(r1: Registers16, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("LD AR16 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r1);
            let value = cpu.registers.get8(r2);
            cpu.mmu.set(address, value);

            cycles(8)
        }))
}

/* Loads a 8 bit immediate value into the memory addressed by r
 */
pub fn ld_ar16_n8(r: Registers16) -> Instruction {
    Instruction::arg8(
        format!("LD AR16 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let address = cpu.registers.get16(r);
            cpu.mmu.set(address, arg as u8);
            cycles(12)
        }))
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 */
pub fn ld_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("LD R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r2);
            let value = cpu.mmu.get(address);
            cpu.registers.set8(r1, value);
            cycles(8)
        }))
}

/* Loads a 8 bit value from the memory addressed by a 16 bit immediate value into r1
 */
pub fn ld_r8_an16(r: Registers8) -> Instruction {
    Instruction::arg16(
        format!("LD R8 AN16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let value = cpu.mmu.get(arg);
            cpu.registers.set8(r, value);
            cycles(16)
        }))
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 * and simultaneously increments r2
 */
pub fn ldi_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("LDI R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r2);
            let value = cpu.mmu.get(address);

            cpu.registers.set8(r1, value);
            cpu.registers.set16(r2, address.wrapping_add(1));

            cycles(8)
        }))
}

/* Loads a 8 bit value from the memory addressed by r2 into r1
 * and simultaneously decements r2
 */
pub fn ldd_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("LDD R7 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r2);
            let value = cpu.mmu.get(address);

            cpu.registers.set8(r1, value);
            cpu.registers.set16(r2, address.wrapping_sub(1));
            cycles(8)
        }))
}

/* Loads a 8 bit value from r2 into the memory addressed by r1
 * and simultaneously increments r1
 */
pub fn ldi_ar16_r8(r1: Registers16, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("LDI AR16 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r1);
            let value = cpu.registers.get8(r2);

            cpu.mmu.set(address, value);
            cpu.registers.set16(r1, address.wrapping_add(1));

            cycles(8)
        }))
}

/* Loads a 8 bit value from r2 into the memory addressed by r1
 * and simultaneously decrements r1
 */
pub fn ldd_ar16_r8(r1: Registers16, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("LDD AR16 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r1);
            let value = cpu.registers.get8(r2);

            cpu.mmu.set(address, value);
            cpu.registers.set16(r1, address.wrapping_sub(1));
            cycles(8)
        }))
}

/* Loads a 8 bit immediate value into r
 */
pub fn ld_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("LD R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            cpu.registers.set8(r, arg as u8);
            cycles(8)
        }))
}

/* Loads a 8 bit value from r2 into r1
 */
pub fn ld_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("LD R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r2);
            cpu.registers.set8(r1, value);
            cycles(4)
        }))
}

/* Loads a 16 bit value from r into the the memory addressed by a 16 bit immediate value
 */
pub fn ld_an16_r16(r: Registers16) -> Instruction {
    Instruction::arg16(
        format!("LD AN16 R16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let value = cpu.registers.get16(r);
            cpu.mmu.set16(arg, value);
            cycles(20)
        }))
}

/* Loads an 8 bit value from r into the the memory addressed by a 16 bit immediate value 
 */
pub fn ld_an16_r8(r: Registers8) -> Instruction {
    Instruction::arg16(
        format!("LD AN16 R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let value = cpu.registers.get8(r);
            cpu.mmu.set(arg, value);
            cycles(16)
        }))
}

/* Loads a 16 bit value from args into the register r
 */
pub fn ld_r16_n16(r: Registers16) -> Instruction {
    Instruction::arg16(
        format!("LD R16 N16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            cpu.registers.set16(r, arg);
            cycles(12)
        }))
}

/* Loads a 16 bit value from r1 into r2
 */
pub fn ld_r16_r16(r1: Registers16, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("LD R16 R16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get16(r1);
            cpu.registers.set16(r2, value);
            cycles(12)
        }))
}

pub fn ld_r16_spn8(r: Registers16) -> Instruction {
    Instruction::arg8(
        format!("LD R16 SPN8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get16(Registers16::SP);
            let v = helper::add_u16_i8(cpu, a, arg as u8);

            cpu.registers.set16(r, v);

            cycles(12)
        }))
}

/* Loads an 8 bit value from r value into the memory at FF00 + an
 */
pub fn ldh_an8_r8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("LDH AN8 R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let v = cpu.registers.get8(r);
            cpu.mmu.set(0xFF00 + arg, v);
            cycles(12)
        }))
}

pub fn ldh_r8_an8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("LDH R8 AN8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let v = cpu.mmu.get(0xFF00 + arg);

            cpu.registers.set8(r, v);
            cycles(12)
        }))
}

pub fn ldc_ar8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("LDC AR8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let ar = cpu.registers.get8(r1) as u16;
            let v = cpu.registers.get8(r2);

            cpu.mmu.set(0xFF00 + ar, v);
            cycles(12)
        }))
}

pub fn ldc_r8_ar8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("LDC R8 AR8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let ar = cpu.registers.get8(r2) as u16;
            let v = cpu.mmu.get(0xFF00 + ar);

            cpu.registers.set8(r1, v);
            cycles(12)
        }))
}

/* Shifts and Rotates */

/* Rotates the A register left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rla() -> Instruction {
    Instruction::no_args(
        format!("RLA"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(Registers8::A);
            let out = helper::rl(cpu, value);
            cpu.registers.set8(Registers8::A, out);
            cpu.registers.set_flag(Flag::Z, false);
            cycles(4)
        }))
}

/* Rotates the register r left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rl_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("RL R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::rl(cpu, value);
            cpu.registers.set8(r, out);
            cycles(8)
        }))
}

/* Rotates memory addressed by register r left through the C register
 * If you have C=1 10001000 and call RLA the result is C=1 00010001
 * C gets treated like as though this were an 9 bit register
 */
pub fn rl_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("RL AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::rl(cpu, value);
            cpu.mmu.set(address, out);
            cycles(16)
        }))
}

/* Rotates the A register right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrca() -> Instruction {
    Instruction::no_args(
        format!("RRCA"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(Registers8::A);
            let out = helper::rrc(cpu, value);
            cpu.registers.set8(Registers8::A, out);
            cpu.registers.set_flag(Flag::Z, false);
            cycles(4)
        }))
}

/* Rotates the register r right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrc_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("RRC R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::rrc(cpu, value);
            cpu.registers.set8(r, out);
            cycles(8)
        }))
}

/* Rotates the memory pointed to by r16 right, puts the shifted bit in c
 * If you have C=1 00010001 and call RRCA the result is C=1 00001000
 * Right most bit is shifted to C but isn't rotated
 */
pub fn rrc_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("RRC AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::rrc(cpu, value);
            cpu.mmu.set(address, out);
            cycles(16)
        }))
}

/* Rotates the A register right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rra() -> Instruction {
    Instruction::no_args(
        format!("RRA"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(Registers8::A);
            let out = helper::rr(cpu, value);
            cpu.registers.set8(Registers8::A, out);
            cpu.registers.set_flag(Flag::Z, false);
            cycles(4)
        }))
}

/* Rotates the register r right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rr_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("RR R8: | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::rr(cpu, value);
            cpu.registers.set8(r, out);
            cycles(9)
        }))
}

/* Rotates the memory pointed to from r right through the C register
 * If you have C=1 10001000 and call RRA the result is C=0 11000100
 * C gets treated like as though this were an 9 bit register
 */
pub fn rr_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("RR AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::rr(cpu, value);
            cpu.mmu.set(address, out);
            cycles(16)
        }))
}

/* Rotates the A register left, puts the shifted bit in c
 * If you have C=1 00010001 and call RLCA the result is C=0 00100010
 * the left most bit is shifted onto C but isn't rotated
 */
pub fn rlca() -> Instruction {
    Instruction::no_args(
        format!("RLCA"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(Registers8::A);
            let out = helper::rlc(cpu, value);
            cpu.registers.set8(Registers8::A, out);
            cpu.registers.set_flag(Flag::Z, false);
            cycles(4)
        }))
}

pub fn rlc_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("RLC R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::rlc(cpu, value);
            cpu.registers.set8(r, out);
            cycles(8)
        }))
}

pub fn rlc_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("RLC AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::rlc(cpu, value);
            cpu.mmu.set(address, out);
            cycles(16)
        }))
}

/* Shift the contents of register r left into Carry. LSB of n set to 0.
 */
pub fn sla_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SLA R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::sla(cpu, value);
            cpu.registers.set8(r, out);
            cycles(8)
        }))
}

/* Shift the memory addressed by r left into Carry. LSB of n set to 0.
 */
pub fn sla_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SLA AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::sla(cpu, value);
            cpu.mmu.set(address, out);
            cycles(16)
        }))
}

/* Shift the contents of register r right into Carry. LSB of n set to 0.
 */
pub fn sra_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SRA R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::sra(cpu, value);
            cpu.registers.set8(r, out);
            cycles(8)
        }))
}

/* Shift the memory addressed by r right into Carry. LSB of n set to 0.
 */
pub fn sra_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SRA AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::sra(cpu, value);
            cpu.mmu.set(address, out);
            cycles(1)
        }))
}

/* Halt CPU & LCD display until button pressed
 */
pub fn stop() -> Instruction {
    Instruction::no_args(
        format!("STOP"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            cpu.stop();
            cycles(4)
        }))
}


/* Halt CPU & LCD display until button pressed
 */
pub fn halt() -> Instruction {
    Instruction::no_args(
        format!("HALT"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            cpu.halt();
            cycles(4)
        }))
}

/* Jumps */

pub fn jp_f_n16(f: JumpFlag) -> Instruction {
    Instruction::arg16(
        format!("JP F | {:?}", f),
        Box::new(move |cpu: &mut CPU, n: u16| {
            /* TODO: note there is a difference in cycle count
             * between matching and not matching branches
             */
            match f {
                JumpFlag::NZ => {
                    if !cpu.registers.get_flag(Flag::Z) {
                        helper::jump(cpu, n);
                    }
                },
                JumpFlag::Z => {
                    if cpu.registers.get_flag(Flag::Z) {
                        helper::jump(cpu, n);
                    }
                },
                JumpFlag::NC => {
                    if !cpu.registers.get_flag(Flag::C) {
                        helper::jump(cpu, n);
                    }
                }
                JumpFlag::C => {
                    if cpu.registers.get_flag(Flag::C) {
                        helper::jump(cpu, n);
                    }
                }
            }

            cycles(12)
        }))
}

pub fn jp_n16() -> Instruction {
    Instruction::arg16(
        format!("JP N16"),
        Box::new(move |cpu: &mut CPU, n: u16| {
            helper::jump(cpu, n);
            cycles(12)
        }))
}

pub fn jp_r16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("JP AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let n = cpu.registers.get16(r);
            helper::jump(cpu, n);
            cycles(4)
        }))
}

pub fn call_n16() -> Instruction {
    Instruction::arg16(
        format!("CALL N16"),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            helper::push(cpu, Registers16::PC);
            helper::jump(cpu, arg);
            cycles(12)
        }))
}

pub fn call_f_n16(f: JumpFlag) -> Instruction {
    Instruction::arg16(
        format!("CALL F N16 | {:?}", f),
        Box::new(move |cpu: &mut CPU, n: u16| {
            match f {
                JumpFlag::NZ => {
                    if !cpu.registers.get_flag(Flag::Z) {
                        helper::call(cpu, n);
                    }
                },
                JumpFlag::Z => {
                    if cpu.registers.get_flag(Flag::Z) {
                        helper::call(cpu, n);
                    }
                },
                JumpFlag::NC => {
                    if !cpu.registers.get_flag(Flag::C) {
                        helper::call(cpu, n);
                    }
                }
                JumpFlag::C => {
                    if cpu.registers.get_flag(Flag::C) {
                        helper::call(cpu, n);
                    }
                }
            }

            cycles(12)
        }))
}

pub fn push_r16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("PUSH R16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            helper::push(cpu, r);
            cycles(16)
        }))
}

pub fn rst_f(f: RstFlag) -> Instruction {
    Instruction::no_args(
        format!("RST F: {:?}", f),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let location = rst_locations(f);
            helper::push(cpu, Registers16::PC);
            helper::jump(cpu, location);
            cycles(32)
        }))
}

pub fn di() -> Instruction {
    Instruction::no_args(
        format!("DI"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            cpu.registers.ime = IME::Disabled;
            cycles(4)
        }))
}

pub fn ei() -> Instruction {
    Instruction::no_args(
        format!("EI"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            cpu.registers.ime = IME::Queued;
            cycles(4)
        }))
}

/* Pop two bytes from stack & jump to that address
 *
 * Note: Jumping is just setting the PC register
 * So we can simplify this function by just passing PC
 * to the helper::pop function that takes the values from the stack
 * and sets them to the given register.
 */
pub fn ret() -> Instruction {
    Instruction::no_args(
        format!("RET"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            helper::ret(cpu);
            cycles(8)
        }))
}

pub fn reti() -> Instruction {
    Instruction::no_args(
        format!("RETI"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            helper::ret(cpu);
            cpu.registers.ime = IME::Queued;
            cycles(8)
        }))
}

pub fn ret_f(f: JumpFlag) -> Instruction {
    Instruction::no_args(
        format!("RET F | {:?}", f),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            match f {
                JumpFlag::NZ => {
                    if !cpu.registers.get_flag(Flag::Z) {
                        helper::ret(cpu);
                    }
                },
                JumpFlag::Z => {
                    if cpu.registers.get_flag(Flag::Z) {
                        helper::ret(cpu);
                    }
                },
                JumpFlag::NC => {
                    if !cpu.registers.get_flag(Flag::C) {
                        helper::ret(cpu);
                    }
                }
                JumpFlag::C => {
                    if cpu.registers.get_flag(Flag::C) {
                        helper::ret(cpu);
                    }
                }
            }

            cycles(8)
        }))
}


pub fn pop_r16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("POP R16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            helper::pop(cpu, r);
            cycles(12)
        }))
}

pub fn jr_n8() -> Instruction {
    Instruction::arg8(
        format!("JR N8"),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            helper::jr(cpu, arg as u8);
            cycles(8)
        }))
}


pub fn jr_f_n8(f: JumpFlag) -> Instruction {
    Instruction::arg8(
        format!("JR F N8 | {:?}", f),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let n = arg as u8;

            match f {
                JumpFlag::NZ => {
                    if !cpu.registers.get_flag(Flag::Z) {
                        helper::jr(cpu, n);
                    }
                },
                JumpFlag::Z => {
                    if cpu.registers.get_flag(Flag::Z) {
                        helper::jr(cpu, n);
                    }
                },
                JumpFlag::NC => {
                    if !cpu.registers.get_flag(Flag::C) {
                        helper::jr(cpu, n);
                    }
                }
                JumpFlag::C => {
                    if cpu.registers.get_flag(Flag::C) {
                        helper::jr(cpu, n);
                    }
                }
            }

            cycles(8)
        }))
}

/* Applies the binary complement to the A register
 */
pub fn cpl() -> Instruction {
    Instruction::no_args(
        format!("CPL"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let v = cpu.registers.get8(Registers8::A);
            cpu.registers.set8(Registers8::A, !v);

            cpu.registers.set_flag(Flag::N, true);
            cpu.registers.set_flag(Flag::H, true);

            cycles(4)
        }))
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
pub fn daa() -> Instruction {
    Instruction::no_args(
        format!("DAA"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
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

            cycles(4)
        }))
}

pub fn scf() -> Instruction {
    Instruction::no_args(
        format!("SCF"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            cpu.registers.set_flag(Flag::N, false);
            cpu.registers.set_flag(Flag::H, false);
            cpu.registers.set_flag(Flag::C, true);

            cycles(4)
        }))
}

pub fn ccf() -> Instruction {
    Instruction::no_args(
        format!("CCF"),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let c = cpu.registers.get_flag(Flag::C);

            cpu.registers.set_flag(Flag::N, false);
            cpu.registers.set_flag(Flag::H, false);
            cpu.registers.set_flag(Flag::C, !c);

            cycles(4)
        }))
}


pub fn add_r16_n8(r: Registers16) -> Instruction {
    Instruction::arg8(
        format!("ADD R16 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get16(r);
            let v = helper::add_u16_i8(cpu, a, arg as u8);

            cpu.registers.set16(r, v);

            cycles(16)
        }))
}

pub fn add_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("ADD R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::add(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(4)
        }))
}

pub fn add_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("ADD R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::add(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn add_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("ADD R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::add(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn adc_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("ADC R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::adc(cpu, a, b);

            cpu.registers.set8(r1, v);

            cycles(4)
        }))
}

pub fn adc_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("ADC R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::adc(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn adc_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("ADC R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::adc(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn add_r16_r16(r1: Registers16, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("ADD R16 R16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get16(r1);
            let b = cpu.registers.get16(r2);

            let (v, overflow) = a.overflowing_add(b);

            cpu.registers.set16(r1, v);

            cpu.registers.set_flag(Flag::C, overflow);
            cpu.registers.set_flag(Flag::N, false);
            cpu.registers.set_flag(Flag::H, bytes::check_half_carry16(a, b));

            cycles(8)
        }))
}


pub fn sub_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SUB R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::sub(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(4)
        }))
}

pub fn sub_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SUB R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::sub(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn sub_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("SUB R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::sub(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn sbc_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SBC R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::sbc(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(4)
        }))
}

pub fn sbc_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SBC R8 R16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::sbc(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn sbc_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("SBC R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::sbc(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn and_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("AND R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::and(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn and_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("AND R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::and(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(4)
        }))
}

pub fn and_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("AND R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::and(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn xor_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("XOR R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::xor(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(4)
        }))
}

pub fn xor_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("XOR R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::xor(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn xor_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("XOR R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::xor(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn or_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("OX R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            let v = helper::or(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(4)
        }))
}

pub fn or_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("OR R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            let v = helper::or(cpu, a, arg as u8);

            cpu.registers.set8(r, v);
            cycles(8)
        }))
}

pub fn or_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("OR R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            let v = helper::or(cpu, a, b);

            cpu.registers.set8(r1, v);
            cycles(8)
        }))
}

pub fn cp_r8_r8(r1: Registers8, r2: Registers8) -> Instruction {
    Instruction::no_args(
        format!("CP R8 R8 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let b = cpu.registers.get8(r2);

            helper::sub(cpu, a, b);

            cycles(4)
        }))
}

pub fn cp_r8_n8(r: Registers8) -> Instruction {
    Instruction::arg8(
        format!("CP R8 N8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, arg: u16| {
            let a = cpu.registers.get8(r);
            helper::sub(cpu, a, arg as u8);
            cycles(8)
        }))
}

pub fn cp_r8_ar16(r1: Registers8, r2: Registers16) -> Instruction {
    Instruction::no_args(
        format!("CP R8 AR16 | {:?} {:?}", r1, r2),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let a = cpu.registers.get8(r1);
            let address = cpu.registers.get16(r2);
            let b = cpu.mmu.get(address);

            helper::sub(cpu, a, b);

            cycles(8)
        }))
}

pub fn swap_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SWAP | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::swap(cpu, value);
            cpu.registers.set8(r, out);
            cycles(8)
        }))
    }

pub fn swap_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SWAP | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::swap(cpu, value);
            cpu.mmu.set(address, out);

            cycles(16)
        }))
    }

pub fn srl_r8(r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SRC R8 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);
            let out = helper::srl(cpu, value);
            cpu.registers.set8(r, out);

            cycles(8)
        }))
}

pub fn srl_ar16(r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SRL AR16 | {:?}", r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);
            let out = helper::srl(cpu, value);
            cpu.mmu.set(address, out);

            cycles(16)
        }))
}

pub fn bit_r8(n:u8, r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("BIT R8 | {:?} {:?}", n, r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);

            helper::bit(cpu, n, value);

            cycles(8)
        }))
}

pub fn bit_ar16(n:u8, r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("BIT AR16 | {:?} {:?}", n, r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);

            helper::bit(cpu, n, value);

            cycles(16)
        }))
}

pub fn res_r8(n:u8, r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("RES R8 | {:?} {:?}", n, r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);

            let out = helper::res(n, value);

            cpu.registers.set8(r, out);

            cycles(8)
        }))
}

pub fn res_ar16(n:u8, r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("RES AR16 | {:?} {:?}", n, r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);

            let out = helper::res(n, value);

            cpu.mmu.set(address, out);

            cycles(16)
        }))
}

pub fn set_r8(n:u8, r: Registers8) -> Instruction {
    Instruction::no_args(
        format!("SET R8 | {:?} {:?}", n, r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let value = cpu.registers.get8(r);

            let out = helper::set(n, value);

            cpu.registers.set8(r, out);

            cycles(8)
        }))
}

pub fn set_ar16(n:u8, r: Registers16) -> Instruction {
    Instruction::no_args(
        format!("SET AR16 | {:?} {:?}", n, r),
        Box::new(move |cpu: &mut CPU, _arg: u16| {
            let address = cpu.registers.get16(r);
            let value = cpu.mmu.get(address);

            let out = helper::set(n, value);

            cpu.mmu.set(address, out);

            cycles(16)
        }))
}

pub fn illegal_opcode(opcode: u32) -> Instruction {
    Instruction::no_args(
        format!("Illegal opcode: 0x{:X}", opcode),
        Box::new(move |_cpu: &mut CPU, _arg: u16| {
            panic!("Illegal opcode!")
        }))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::rom::BootRom;
    use crate::cartridge::Cartridge;
    use crate::register::Registers;
    use crate::mmu::MMU;

    fn test_cpu() -> CPU {
        CPU::new(Registers::new(), MMU::new(BootRom::zero(), Cartridge::zero()))
    }

    #[test]
    fn test_bit_r8() {
        let mut cpu = test_cpu();

        helper::bit(&mut cpu, 7, 0x80);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
    }

    #[test]
    fn test_set_r8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x80);
        cpu.execute(&set_r8(3, Registers8::A));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x88);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_scf() {
        let mut cpu = test_cpu();

        cpu.execute(&scf());

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_adc_r8_r8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0xE1);
        cpu.registers.set8(Registers8::E, 0x0F);
        cpu.registers.set_flag(Flag::C, true);

        cpu.execute(&adc_r8_r8(Registers8::A, Registers8::E));

        assert_eq!(cpu.registers.get8(Registers8::A), 0xF1);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_adc_r8_n8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0xE1);
        cpu.registers.set_flag(Flag::C, true);
        cpu.push_pc(0xFF80, 0x3B);

        cpu.execute(&adc_r8_n8(Registers8::A));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x1D);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_sdc_r8_r8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x3B);
        cpu.registers.set8(Registers8::H, 0x2A);
        cpu.registers.set_flag(Flag::C, true);

        cpu.execute(&sbc_r8_r8(Registers8::A, Registers8::H));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x10);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_dec_r8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::L, 0x01);

        cpu.execute(&dec_r8(Registers8::L));

        assert_eq!(cpu.registers.get8(Registers8::L), 0x00);

        assert_eq!(cpu.registers.get_flag(Flag::Z), true);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        // interestingly blarggs says inc and dec nevery set carry
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_inc_r8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::L, 0x01);

        cpu.execute(&inc_r8(Registers8::L));

        assert_eq!(cpu.registers.get8(Registers8::L), 0x02);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_rlca() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x85);

        cpu.execute(&rlca());

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
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x8A);

        cpu.execute(&sra_r8(Registers8::A));

        assert_eq!(cpu.registers.get8(Registers8::A), 0xC5);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_dec_ar16() {
        let mut cpu = test_cpu();

        cpu.mmu.set(0xFF80, 0x00);
        cpu.registers.set16(Registers16::HL, 0xFF80);
        cpu.registers.set_flag(Flag::C, true);

        cpu.execute(&dec_ar16(Registers16::HL));

        assert_eq!(cpu.mmu.get(0xFF80), 0xFF);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);

        /* assert that C isn't affected */
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_inc_ar16() {
        let mut cpu = test_cpu();

        cpu.mmu.set(0xFF80, 0x50);
        cpu.registers.set16(Registers16::HL, 0xFF80);
        cpu.registers.set_flag(Flag::C, true);

        cpu.execute(&inc_ar16(Registers16::HL));

        assert_eq!(cpu.mmu.get(0xFF80), 0x51);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);

        /* assert that C isn't affected */
        assert_eq!(cpu.registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_daa() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x45);
        cpu.registers.set8(Registers8::B, 0x38);
        cpu.registers.set_flag(Flag::N, true);

        cpu.execute(&add_r8_r8(Registers8::A, Registers8::B));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x7D);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);

        cpu.execute(&daa());

        assert_eq!(cpu.registers.get8(Registers8::A), 0x83);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

        cpu.execute(&sub_r8_r8(Registers8::A, Registers8::B));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x4B);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);

        cpu.execute(&daa());

        assert_eq!(cpu.registers.get8(Registers8::A), 0x45);
    }

    #[test]
    fn test_ld_r16_spn8() {
        let mut cpu = test_cpu();

        cpu.registers.set16(Registers16::SP, 0xFFF8);
        cpu.push_pc(0xFF80, 0x02);

        cpu.execute(&ld_r16_spn8(Registers16::HL));

        assert_eq!(cpu.registers.get16(Registers16::HL), 0xFFFA);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

    }

    #[test]
    fn test_ld_r16_spn8_sub() {
        let mut cpu = test_cpu();

        cpu.registers.set16(Registers16::SP, 0xDFFD);
        cpu.push_pc(0xFF80, 0xFE);

        cpu.execute(&ld_r16_spn8(Registers16::HL));

        assert_eq!(cpu.registers.get16(Registers16::HL), 0xDFFB);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

    }

    #[test]
    fn test_add_r16_n8() {
        let mut cpu = test_cpu();

        cpu.registers.set16(Registers16::SP, 0xDFFD);
        cpu.push_pc(0xFF80, 0x01);

        cpu.execute(&add_r16_n8(Registers16::SP));

        assert_eq!(cpu.registers.get16(Registers16::SP), 0xDFFE);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_add_r16_n8_sub() {
        let mut cpu = test_cpu();

        cpu.registers.set16(Registers16::SP, 0xDFFD);
        cpu.push_pc(0xFF80, 0xFF);

        cpu.execute(&add_r16_n8(Registers16::SP));

        assert_eq!(cpu.registers.get16(Registers16::SP), 0xDFFC);
        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_jp_f_n16() {
        let mut cpu = test_cpu();

        cpu.registers.set_flag(Flag::Z, true);
        cpu.push_pc(0xFF80, 0x10);

        cpu.execute(&jp_f_n16(JumpFlag::Z));

        assert_eq!(cpu.registers.get16(Registers16::PC), 0x10);
    }

    #[test]
    fn test_jp_f_n16_no_test() {
        let mut cpu = test_cpu();

        cpu.registers.set_flag(Flag::Z, true);
        cpu.push_pc(0xFF80, 0x10);

        cpu.execute(&jp_f_n16(JumpFlag::NZ));

        assert_eq!(cpu.registers.get16(Registers16::PC), 0xFF82);
    }

    #[test]
    fn test_call() {
        let mut cpu = test_cpu();

        cpu.push_pc(0x8002, 0x12);
        cpu.push_pc(0x8001, 0x34);
        cpu.registers.set16(Registers16::SP, 0xFFFE);

        cpu.execute(&call_n16());

        assert_eq!(cpu.registers.get16(Registers16::PC), 0x1234);
        assert_eq!(cpu.registers.get16(Registers16::SP), 0xFFFC);

        assert_eq!(cpu.mmu.get(0xFFFD), 0x80);
        assert_eq!(cpu.mmu.get(0xFFFC), 0x03);
    }

    #[test]
    fn test_ret() {
        let mut cpu = test_cpu();

        cpu.push_pc(0x8002, 0x90);
        cpu.push_pc(0x8001, 0x00);

        cpu.execute(&call_n16());

        assert_eq!(cpu.registers.get16(Registers16::PC), 0x9000);

        cpu.execute(&ret());

        assert_eq!(cpu.registers.get16(Registers16::PC), 0x8003);
    }

    #[test]
    fn test_sub_r8_n8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x3E);
        cpu.registers.set8(Registers8::E, 0x3E);

        cpu.execute(&sub_r8_r8(Registers8::A, Registers8::E));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x00);

        assert_eq!(cpu.registers.get_flag(Flag::Z), true);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);

    }

    #[test]
    fn test_cp_r8_r8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x3C);
        cpu.registers.set8(Registers8::B, 0x2F);

        cpu.execute(&cp_r8_r8(Registers8::A, Registers8::B));

        assert_eq!(cpu.registers.get8(Registers8::A), 0x3C);

        assert_eq!(cpu.registers.get_flag(Flag::Z), false);
        assert_eq!(cpu.registers.get_flag(Flag::H), true);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }

    #[test]
    fn test_cp_r8_n8() {
        let mut cpu = test_cpu();

        cpu.registers.set8(Registers8::A, 0x3C);
        cpu.push_pc(0xFF80, 0x3C);

        cpu.execute(&cp_r8_n8(Registers8::A));

        assert_eq!(cpu.registers.get_flag(Flag::Z), true);
        assert_eq!(cpu.registers.get_flag(Flag::H), false);
        assert_eq!(cpu.registers.get_flag(Flag::N), true);
        assert_eq!(cpu.registers.get_flag(Flag::C), false);
    }
}
