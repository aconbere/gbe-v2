use std::io;
use std::io::{BufRead, Error, ErrorKind, stdout, Write};
use std::str::SplitWhitespace;
use std::sync::mpsc::Sender;

use crate::cpu::Target;

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
/// > print PC
/// PC: 0xCB01
///
/// > print Z
///
/// Z: 0
/// 
/// > print 0xFF80
///
/// 0x010C
///
/// > print
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
/// ## delete 
///
/// Deletes a breakpoint
///
/// > delete 0x100
/// > list
/// 0xFF80

type ParseResult<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidFlag(String),
    InvalidCommand(String),
    InvalidArguement(Token),
    UnknownArgumentType(String),
    InvalidEndOfInput,
    InvalidRegister(String),
    InvalidHexString(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Break,
    List,
    Print,
    Run,
    Step,
    Next,
    Finish,
    Delete,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Command(Command),
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

pub fn parse_flag(input: &str) -> ParseResult<Flag> {
    match input {
        "Z" => Ok(Flag::Z),
        "C" => Ok(Flag::C),
        "N" => Ok(Flag::N),
        "H" => Ok(Flag::H),
        _ => Err(ParserError::InvalidFlag(input.to_string()))
    }
}

pub fn parse_register(input: &str) -> ParseResult<Register> {
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
        _ => Err(ParserError::InvalidRegister(input.to_string())),
    }
}

fn _error(error: String) -> Error {
    Error::new(ErrorKind::Other, error)
}

fn parse_arguement(input: &str) -> ParseResult<Token> {
    if input.starts_with("f") {
        let flag = parse_flag(&input[1..])?;
        Ok(Token::Flag(flag))
    } else if input.starts_with("r") {
        let register = parse_register(&input[1..])?;
        Ok(Token::Register(register))
    } else if input.starts_with("0x") {
        match u16::from_str_radix(&input[2..], 16) {
            Ok(register) => Ok(Token::Address(register)),
            e => Err(ParserError::InvalidHexString(input.to_string())),
        }
    } else {
        Err(ParserError::UnknownArgumentType(input.to_string()))
    }
}

fn parse_command(input: &str) -> ParseResult<Command> {
    match input {
        "b" | "break" => Ok(Command::Break),
        "p" | "print" => Ok(Command::Print),
        "l" | "list" => Ok(Command::List),
        "r" | "run" => Ok(Command::Run),
        "s" | "step" => Ok(Command::Step),
        "n" | "next" => Ok(Command::Next),
        "f" | "finish" => Ok(Command::Finish),
        "d" | "delete" => Ok(Command::Delete),
        _ => Err(ParserError::InvalidCommand(input.to_string()))
    }
}

/* Takes input (stream of text split on whitespace) and cosumes that Stream turning them into a
 * Vector of Tokens or Errors
 */
pub fn tok(input: &mut SplitWhitespace, output: &mut Vec<Token>) -> ParseResult<bool> {
    _tok(input, State::Command, output)
}

pub fn _tok(input: &mut SplitWhitespace, state: State, tokens: &mut Vec<Token>) -> ParseResult<bool> {
    match input.next() {
        /* If we have nothing left in our input itterator then we are done. return our
         * completed output */
        None => Ok(true),
        Some(untok) => {
            /* Take the next white space seperated string and match it */
            match state {
                State::Argument => {
                    let tok = parse_arguement(untok)?;
                    tokens.push(tok);
                    _tok(input, State::Argument, tokens)
                },
                State::Command => {
                    let command = parse_command(untok)?;
                    tokens.push(Token::Command(command));
                    /* Need to handle break, print and delete which take arguments */
                    _tok(input, State::Argument, tokens)
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
    Target(Target),
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

    pub fn eval(&mut self, tokens: Vec<Token>) -> ParseResult<Output> {
        let first_token = tokens[0];

        match first_token {
            Token::Command(command) => {
                match command {
                    Command::Break => {
                        let arg = tokens[1];
                        match arg {
                            Token::Address(a) => {
                                self.set(a);
                                Ok(Output::AddressList(self.list()))
                            },
                            _ => Err(ParserError::InvalidArguement(arg))
                        }
                    },
                    Command::List => {
                        Ok(Output::AddressList(self.list()))
                    }
                    Command::Print => {
                        let arg = tokens.get(1);

                        match arg {
                            Some(Token::Register(r)) => {
                                Ok(Output::Text(self.print_register(*r)))
                            },
                            Some(Token::Flag(f)) => {
                                Ok(Output::Text(self.print_flag(*f)))
                            },
                            Some(a) => {
                                Err(ParserError::InvalidArguement(*a))
                            },
                            None => {
                                Ok(Output::Text(self.print_all()))
                            }
                        }
                    },
                    Command::Run => {
                        self.run();
                        Ok(Output::Unit)
                    },
                    Command::Step => {
                        self.step();
                        Ok(Output::Unit)
                    }
                    Command::Next => {
                        self.next();
                        Ok(Output::Unit)
                    },
                    Command::Finish => {
                        self.finish();
                        Ok(Output::Unit)
                    }
                    Command::Delete => {
                        let arg = tokens[1];
                        match arg {
                            Token::Address(a) => {
                                self.delete(a);
                                Ok(Output::AddressList(self.break_points.clone()))
                            },
                            _ => Err(ParserError::InvalidArguement(arg))
                        }
                    }
                }
            }
        }
    }
}


pub fn start(debugger_sender: Sender<Target>) {
    let stdin = io::stdin();

    let mut input_handle = stdin.lock();
    let mut output_handle = stdout();
    let mut debugger = Debugger::new();

    loop {
        output_handle.write(prompt().as_bytes()).unwrap();
        output_handle.flush().unwrap();

        match read(&mut input_handle) {
            Ok(tokens) => {
                let output = debugger.eval(tokens).unwrap();
            },
            e => println!("Error: {:?}", e)
        }
    }

}


fn prompt() -> String {
    "> ".to_string()
}

fn read(buffer: &mut dyn BufRead) -> ParseResult<Vec<Token>> {
    let mut input = String::new();
    match buffer.read_line(&mut input) {
        Ok(_) => {
            let mut output = &mut Vec::new();
            tok(&mut input.trim().split_whitespace(), &mut output)?;
            Ok(*output)
        }
        Err(_) => Err(ParserError::InvalidEndOfInput),
    }
}
