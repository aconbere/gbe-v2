use std::io;
use std::io::{BufRead, Error, ErrorKind, stdout, Write};
use std::str::SplitWhitespace;

/// # Mini Debugger Language
///
/// ## break <a16>
///
/// Set a break point at a16 a 16 bit address register
///
/// > break 0x0100
///
/// ## list
///
/// Lists all set break points
///
/// > list
/// 0x0100
/// 0xFF80
///
/// ## run
///
/// Runs until a breakpoint is reached or the end of the program
///
/// ## print [r|f|a16]
///
/// Prints the value of a register, flag, or memory address. With no arguments prints all registers.
///
/// > p PC
/// PC: 0xCB01
///
/// > p Z
///
/// Z: 0
/// 
/// > p 0xFF80
///
/// 0x010C
///
/// > p
///
/// AF: 0x010C
/// BC: 0x0000
/// DE: 0x56AB
/// HL: 0x1234
/// PC: 0x010C
/// SP: 0x1111
/// Z:0 H:1 N:1 C:0
/// IME: 0
///
///
/// ## step
///
/// Advances to the next instruction, if a call is issued will follow the call
///
/// > step
/// PC: 0x01CD; NOP
///
/// ## next
///
/// Advances to the next instruction, if it's a call will continue execution until return
///
/// > next
/// PC: 0x01DE; INC B
///
/// ## finish
///
/// Advances until the end of the current function
///
/// > finish
/// PC: 0x01CD; RET
///
///
/// ## delete 
///
/// Deletes a breakpoint
///
/// > delete 0x100
/// > list
/// 0xFF80

#[derive(Debug, Clone, Copy)]
pub enum DebuggerError {
    InvalidFlag,
    InvalidEndOfInput,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Break,
    List,
    Print,
    Run,
    Step,
    Next,
    Finish,
    Delete,
    Address(u16),
    Register(Register),
    Flag(Flag),
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    A, B, C, D, E, F, H, L,
    AF, BC, DE, HL, PC, SP,
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Z, C, N, H,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Command,
    Argument,
}


pub fn parse_flag(input: &str) -> Result<Flag, Error> {
    match input {
        "Z" => Ok(Flag::Z),
        "C" => Ok(Flag::C),
        "N" => Ok(Flag::N),
        "H" => Ok(Flag::H),
        _ => Err(Error::new(ErrorKind::Other, format!("invalid flag: {}", input)))
    }
}

pub fn parse_register(input: &str) -> Result<Register, Error> {
    match input {
        "A" => Ok(Register::A),
        "B" => Ok(Register::B),
        "C" => Ok(Register::C),
        "D" => Ok(Register::D),
        "E" => Ok(Register::E),
        "F" => Ok(Register::F),
        "H" => Ok(Register::H),
        "L" => Ok(Register::L),

        "AF" => Ok(Register::AF),
        "BC" => Ok(Register::BC),
        "DE" => Ok(Register::DE),
        "HL" => Ok(Register::HL),
        "SP" => Ok(Register::SP),
        "PC" => Ok(Register::PC),
        _ => Err(Error::new(ErrorKind::Other, format!("invalid register: {}", input)))
    }
}

fn _error(error: String) -> Error {
    Error::new(ErrorKind::Other, error)
}

pub fn tok(input: &mut SplitWhitespace) -> Result<Vec<Token>, Error> {
    _tok(input, State::Command, &Vec::new())
}

pub fn _tok(input: &mut SplitWhitespace, state: State, _output: &Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut output = _output.clone();

    match input.next() {
        None => Ok(output),
        Some(untok) => {
            match state {
                State::Argument => {
                    if untok.starts_with("f") {
                        let flag = parse_flag(&untok[1..])?;
                        output.push(Token::Flag(flag));
                        Ok(output)
                    } else if untok.starts_with("r") {
                        let register = parse_register(&untok[1..])?;
                        output.push(Token::Register(register));
                        Ok(output)
                    } else if untok.starts_with("0x") {
                        match u16::from_str_radix(&untok[2..], 16) {
                            Ok(register) => {
                                output.push(Token::Address(register));
                                Ok(output)
                            }
                            e => Err(_error(format!("Invalid hex string: {} {:?}", untok, e)))
                        }
                    } else {
                        Err(_error(format!("Invalid argument: {}", untok)))
                    }
                }

                State::Command => {
                    match untok {
                        "b" | "break" => {
                            output.push(Token::Break);
                            _tok(input, State::Argument, &output)
                        }
                        "p" | "print" => {
                            output.push(Token::Print);
                            _tok(input, State::Argument, &output)
                        }
                        "l" | "list" => {
                            output.push(Token::List);
                            Ok(output)
                        }
                        "r" | "run" => {
                            output.push(Token::Run);
                            Ok(output)
                        }
                        "s" | "step" => {
                            output.push(Token::Step);
                            Ok(output)
                        }
                        "n" | "next" => {
                            output.push(Token::Next);
                            Ok(output)
                        }
                        "f" | "finish" => {
                            output.push(Token::Finish);
                            Ok(output)
                        }
                        "d" | "delete" => {
                            output.push(Token::Delete);
                            _tok(input, State::Argument, &output)
                        }

                        _ => {
                            Err(_error(format!("Invalid command: {}", untok)))
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Registers {
    af: u16
}

#[derive(Debug)]
pub enum Output {
    Address(u16),
    AddressList(Vec<u16>),
    Registers(Registers),
    Text(String),
    Unit,
}

struct Debugger {
    break_points: Vec<u16>,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            break_points: Vec::new(),
        }
    }

    pub fn set(&mut self, pc: u16) {
        if !self.break_points.contains(&pc) {
            self.break_points.push(pc);
        }
    }

    pub fn list(&self) -> Vec<u16> {
        self.break_points.clone()
    }

    pub fn run(&self) {
    }

    pub fn step(&self) {
    }

    pub fn next(&self) {
    }

    pub fn finish(&self) {
    }

    pub fn delete(&mut self, pc: u16) {
        self.break_points.retain(|e| *e != pc);
    }

    pub fn print_all(&self) -> String {
        String::from("wtf")
    }

    pub fn print_register(&self, r: Register) -> String {
        match r {
            Register::AF => format!("{}", "AF"),
            _ => format!("{}", "??")
        }
    }

    pub fn print_flag(&self, f: Flag) -> String {
        match f {
            Flag::Z => format!("{}", "fZ"),
            _ => format!("{}", "f?")
        }
    }

    pub fn eval(&mut self, tokens: Vec<Token>) -> Result<Output, Error> {
        let command = tokens[0];

        match command {
            Token::Break => {
                let arg = tokens[1];
                match arg {
                    Token::Address(a) => {
                        self.set(a);
                        Ok(Output::Unit)
                    },
                    _ => Err(_error(format!("Invalid argument to break: {:?}", arg)))
                }
            },
            Token::List => Ok(Output::AddressList(self.list())),
            Token::Print => {
                let arg = tokens.get(1);
                match arg {
                    Some(Token::Register(r)) => {
                        Ok(Output::Text(self.print_register(*r)))
                    },
                    Some(Token::Flag(f)) => {
                        Ok(Output::Text(self.print_flag(*f)))
                    },
                    Some(_) => {
                        Err(_error(format!("Invalid argument to print: {:?}", arg)))
                    },
                    None => {
                        Ok(Output::Text(self.print_all()))
                    }
                }
            },
            Token::Run => {
                self.run();
                Ok(Output::Unit)
            },
            Token::Step => {
                self.step();
                Ok(Output::Unit)
            }
            Token::Next => {
                self.next();
                Ok(Output::Unit)
            },
            Token::Finish => {
                self.finish();
                Ok(Output::Unit)
            }
            Token::Delete => {
                let arg = tokens[1];
                match arg {
                    Token::Address(a) => {
                        self.delete(a);
                        Ok(Output::AddressList(self.break_points.clone()))
                    },
                    _ => Err(_error(format!("Invalid argument to break: {:?}", arg)))
                }
            }
            _ => Err(_error(format!("Invalid command: {:?}", command)))
        }
    }
}

pub fn start(debugger_sender: SyncSender<Target>) {
    let stdin = io::stdin();

    let mut input_handle = stdin.lock();
    let mut output_handle = stdout();
    let mut debugger = Debugger::new();

    loop {
        output_handle.write(prompt().as_bytes()).unwrap();
        output_handle.flush().unwrap();

        match read(&mut input_handle) {
            Ok(tokens) => {
                println!("Tokens: {:?}", tokens);
                let output = debugger.eval(tokens).unwrap();
                println!("output: {:?}", output);
            },
            e => println!("Error: {:?}", e)
        }
    }

}


fn prompt() -> String {
    "> ".to_string()
}

fn read(buffer: &mut dyn BufRead) -> Result<Vec<Token>, Error> {
    let mut input = String::new();
    buffer.read_line(&mut input)?;

    Ok(tok(&mut input.trim().split_whitespace())?)
}
