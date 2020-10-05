use std::io;
use std::io::{BufRead, Error, ErrorKind, stdout, Write};
use std::str::SplitWhitespace;

type ParseResult<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Clone)]
pub enum ParserError {
    InvalidFlag(String),
    InvalidCommand(String),
    InvalidArgument(Argument),
    UnknownArgumentType(String),
    InvalidEndOfInput,
    InvalidRegister(String),
    InvalidHexString(String),
}

fn parse_hex(input: &str) -> ParseResult<u16> {
    if input.starts_with("0x") {
        u16::from_str_radix(&input[2..], 16)
            .or(Err(ParserError::InvalidHexString(input.to_string())))
    } else {
        Err(ParserError::UnknownArgumentType(input.to_string()))
    }
}

pub trait CommandT {
    fn eval(&self, d: &mut Debugger) -> ParseResult<Output>;
}

pub struct Break {
    point: u16,
}

impl Break {
    fn new(stream: &mut SplitWhitespace) -> ParseResult<Break> {
        let input = stream.next().ok_or(ParserError::InvalidEndOfInput)?;
        Ok(Break { point: parse_hex(input)? })
    }
}

impl CommandT for Break {
    fn eval(&self, d: &mut Debugger) -> ParseResult<Output> {
        d.set_break(self.point);
        Ok(Output::Void)
    }
}

pub struct Delete {
    point: u16,
}

impl Delete {
    pub fn new(stream: &mut SplitWhitespace) -> ParseResult<Delete> {
        let input = stream.next().ok_or(ParserError::InvalidEndOfInput)?;
        Ok(Delete { point: parse_hex(input)? })
    }
}

impl CommandT for Delete {
    fn eval(&self, d: &mut Debugger) -> ParseResult<Output> {
        d.delete(self.point);
        Ok(Output::Void)
    }
}

pub struct List { }

impl CommandT for List {
    fn eval(&self, _: &mut Debugger) -> ParseResult<Output> {
        Ok(Output::Void)
    }
}

impl List {
    pub fn new(_: &mut SplitWhitespace) -> ParseResult<List> {
        Ok(List {})
    }
}

pub struct Print { }

impl CommandT for Print {
    fn eval(&self, _: &mut Debugger) -> ParseResult<Output> {
        Ok(Output::Void)
    }
}

impl Print {
    fn print_reference(&self, arg: Argument) -> Output {
        match arg {
            Argument::Register(r) => self.print_register(r),
            Argument::Flag(r) => self.print_flag(r),
            Argument::Address(r) => self.print_address(r),
        };

        Output::Void
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

    pub fn print_address(&self, a: u16) -> String {
        format!("{}", "f?")
    }
    pub fn new(_: &mut SplitWhitespace) -> ParseResult<List> {
        Ok(List {})
    }
}

pub struct Next { }

impl CommandT for Next {
    fn eval(&self, _: &mut Debugger) -> ParseResult<Output> {
        Ok(Output::Void)
    }
}

impl Next {
    pub fn new(_: &mut SplitWhitespace) -> ParseResult<List> {
        Ok(List {})
    }
}

pub struct Step { }

impl CommandT for Step {
    fn eval(&self, _: &mut Debugger) -> ParseResult<Output> {
        Ok(Output::Void)
    }
}

impl Step {
    pub fn new(_: &mut SplitWhitespace) -> ParseResult<Step> {
        Ok(Step {})
    }
}

pub struct Run { }

impl CommandT for Run {
    fn eval(&self, _: &mut Debugger) -> ParseResult<Output> {
        Ok(Output::Void)
    }
}

impl Run {
    pub fn new(_: &mut SplitWhitespace) -> ParseResult<Run> {
        Ok(Run {})
    }
}

pub struct Finish { }

impl CommandT for Finish {
    fn eval(&self, _: &mut Debugger) -> ParseResult<Output> {
        Ok(Output::Void)
    }
}

impl Finish {
    pub fn new(_: &mut SplitWhitespace) -> ParseResult<Finish> {
        Ok(Finish {})
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Argument {
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

fn parse_flag(input: &str) -> ParseResult<Flag> {
    match input {
        "Z" => Ok(Flag::Z),
        "C" => Ok(Flag::C),
        "N" => Ok(Flag::N),
        "H" => Ok(Flag::H),
        _ => Err(ParserError::InvalidFlag(input.to_string()))
    }
}

fn parse_register(input: &str) -> ParseResult<Register> {
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

fn parse_argument(stream: &mut SplitWhitespace) -> ParseResult<Argument> {
    let input = stream.next().ok_or(ParserError::InvalidEndOfInput)?;

    if input.starts_with("f") {
        let flag = parse_flag(&input[1..])?;
        Ok(Argument::Flag(flag))
    } else if input.starts_with("r") {
        let register = parse_register(&input[1..])?;
        Ok(Argument::Register(register))
    } else if input.starts_with("0x") {
        match u16::from_str_radix(&input[2..], 16) {
            Ok(register) => Ok(Argument::Address(register)),
            _ => Err(ParserError::InvalidHexString(input.to_string())),
        }
    } else {
        Err(ParserError::UnknownArgumentType(input.to_string()))
    }
}

fn parse_arguments(input: &mut SplitWhitespace, count: u8) -> ParseResult<Vec<Argument>> {
    let mut output = Vec::new();

    for _ in 0..count {
        output.push(parse_argument(input)?);
    }

    Ok(output)
}

fn parse_address(input: &mut SplitWhitespace) -> ParseResult<Argument> {
    let a = parse_arguments(input, 1)?[0];

    match a {
        Argument::Address(_) => Ok(a),
        _ => Err(ParserError::InvalidArgument(a))
    }
}

fn parse_command(stream: &mut SplitWhitespace) -> ParseResult<Box<dyn CommandT>> {
    let input = stream.next().ok_or(ParserError::InvalidEndOfInput)?;

    match input {
        "b" | "break" => Ok(Box::new(Break::new(stream)?)),
        "d" | "delete" => Ok(Box::new(Delete::new(stream)?)),
        "p" | "print" =>  Ok(Box::new(Print::new(stream)?)),
        "l" | "list" =>  Ok(Box::new(List::new(stream)?)),
        "r" | "run" =>  Ok(Box::new(Run::new(stream)?)),
        "s" | "step" =>  Ok(Box::new(Step::new(stream)?)),
        "n" | "next" =>  Ok(Box::new(Next::new(stream)?)),
        "f" | "finish" => Ok(Box::new(Finish::new(stream)?)),
        _ => Err(ParserError::InvalidCommand(input.to_string()))
    }
}

#[derive(Debug)]
pub struct Registers {
    af: u16
}

#[derive(Debug)]
pub enum Output {
    // Address(u16),
    // AddressList(Vec<u16>),
    // Registers(Registers),
    // Text(String),
    Void,
    Error(String),
}

pub struct Debugger {
    break_points: Vec<u16>,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            break_points: Vec::new(),
        }
    }

    pub fn list(&self) -> Output {
        Output::Void
    }

    pub fn run(&self) -> Output {
        Output::Void
    }

    pub fn step(&self) -> Output {
        Output::Void
    }

    pub fn next(&self) -> Output {
        Output::Void
    }

    pub fn finish(&self) -> Output {
        Output::Void
    }

    pub fn delete(&mut self, pc: u16) -> Output {
        self.break_points.retain(|e| *e != pc);
        Output::Void
    }

    fn set_break(&mut self, pc: u16) -> Output {
        if !self.break_points.contains(&pc) {
            self.break_points.push(pc);
        }

        Output::Void
    }

}

fn prompt() -> String {
    "> ".to_string()
}

fn read(buffer: &mut dyn BufRead) -> ParseResult<Box<dyn CommandT>> {
    let mut input = String::new();
    buffer.read_line(&mut input).or(Err(ParserError::InvalidEndOfInput))?;
    parse_command(&mut input.trim().split_whitespace())
}

pub fn start() -> ParseResult<()>{
    let stdin = io::stdin();

    let mut input_handle = stdin.lock();
    let mut output_handle = stdout();
    let mut debugger = Debugger::new();

    loop {
        output_handle.write(prompt().as_bytes()).unwrap();
        output_handle.flush().unwrap();

        let command = read(&mut input_handle)?;
        let output = command.eval(&mut debugger)?;
        println!("\t {:?}", output);
    }
}
