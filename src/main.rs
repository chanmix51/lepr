extern crate ansi_term;

use ansi_term::Colour;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Parser)]
#[grammar = "boolean_expression.pest"]
pub struct BEParser;

pub struct Registers {
    accumulator: u8,
    register_x: u8,
    register_y: u8,
    status_register: u8,
    stack_pointer: u8,
    command_pointer: usize,
}

impl Registers {
    pub fn new(start_addr: usize) -> Registers {
        Registers {
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            status_register: 0b00110000,
            stack_pointer: 0,
            command_pointer: start_addr,
        }
    }
}

#[derive(Debug)]
pub enum Source8 {
    Accumulator,
    RegisterX,
    RegisterY,
    RegisterS,
    RegisterSP,
    Memory(usize),
}

impl Source8 {
    pub fn get_value(&self, registers: &Registers, memory: &Vec<u8>) -> u8 {
        match *self {
            Source8::Accumulator => registers.accumulator,
            Source8::RegisterX => registers.register_x,
            Source8::RegisterY => registers.register_y,
            Source8::RegisterSP => registers.status_register,
            Source8::RegisterS => registers.stack_pointer,
            Source8::Memory(addr) => memory[addr],
        }
    }
}

#[derive(Debug)]
pub enum BooleanExpression {
    Equal(Source8, u8),
    GreaterOrEqual(Source8, u8),
    StrictlyGreater(Source8, u8),
    LesserOrEqual(Source8, u8),
    StrictlyLesser(Source8, u8),
    Different(Source8, u8),
    Value(bool),
}

impl BooleanExpression {
    pub fn solve(&self, registers: &Registers, memory: &Vec<u8>) -> bool {
        match &*self {
            BooleanExpression::Equal(source, val) => source.get_value(registers, memory) == *val,
            BooleanExpression::GreaterOrEqual(source, val) => {
                source.get_value(registers, memory) >= *val
            }
            BooleanExpression::StrictlyGreater(source, val) => {
                source.get_value(registers, memory) > *val
            }
            BooleanExpression::LesserOrEqual(source, val) => {
                source.get_value(registers, memory) <= *val
            }
            BooleanExpression::StrictlyLesser(source, val) => {
                source.get_value(registers, memory) < *val
            }
            BooleanExpression::Different(source, val) => {
                source.get_value(registers, memory) != *val
            }
            BooleanExpression::Value(val) => *val,
        }
    }
}

fn get_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn display_error<T>(line: &String, err: PestError<T>) {
    let (mark_str, msg) = match err.location {
        pest::error::InputLocation::Pos(x)  => {
            let mut pos_str = String::new();
            for _ in 0..x { pos_str.push(' '); }
            pos_str.push('↑');

            (pos_str, format!("at position {}", x))
        },
        pest::error::InputLocation::Span((a, b)) => {
            let mut pos_str = String::new();
            for _ in 0..a { pos_str.push(' '); }
            pos_str.push('↑');
            for _ in a..b { pos_str.push(' '); }
            pos_str.push('↑');
            (pos_str, format!("somewhere between position {} and {}", a, b))
        },
    };
    println!(
        "Syntax error:\n{}.\n{}\n{}",
        line,
        mark_str,
        msg
    );
}

fn main() {
    println!("{}", Colour::Green.paint("Welcome in Lepr 0.1.0"));
    let prompt = format!("{}", Colour::Yellow.paint(">> "));
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                match BEParser::parse(Rule::boolean_expression, line.as_str()) {
                    Ok(mut pairs)   => {
                        let response = parse_boolex(pairs.next().unwrap().into_inner());
                        println!("{:?}", response);
                    },
                    Err(parse_err)    => display_error(&line, parse_err),
                };
            },
            Err(ReadlineError::Eof) => {
                println!("Quit!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

pub fn parse_boolex(mut nodes: Pairs<Rule>) -> BooleanExpression {
    let node = nodes.next().unwrap();
    match node.as_rule() {
        Rule::boolean => BooleanExpression::Value(node.as_str() == "true"),
        Rule::operation => parse_operation(node.into_inner()),
        smt => panic!("unknown node type '{:?}'.", smt),
    }
}

fn parse_operation(mut nodes: Pairs<Rule>) -> BooleanExpression {
    let node = nodes.next().unwrap();
    let lh = match node.as_rule() {
        Rule::register8 => parse_register(&node),
        Rule::memory => parse_memory(&node),
        v => panic!("unexpected node '{:?}' here.", v),
    };
    let middle_node = nodes.next().unwrap();
    let node = nodes.next().unwrap();
    let rh = parse_value8(&node);
    match middle_node.as_str() {
        "=" => BooleanExpression::Equal(lh, rh),
        ">=" => BooleanExpression::GreaterOrEqual(lh, rh),
        ">" => BooleanExpression::StrictlyGreater(lh, rh),
        "<=" => BooleanExpression::LesserOrEqual(lh, rh),
        "<" => BooleanExpression::StrictlyLesser(lh, rh),
        "!=" => BooleanExpression::Different(lh, rh),
        v => panic!("unknown 8 bits provider {:?}", v),
    }
}

fn parse_register(node: &Pair<Rule>) -> Source8 {
    match node.as_str() {
        "A" => Source8::Accumulator,
        "X" => Source8::RegisterX,
        "Y" => Source8::RegisterY,
        "S" => Source8::RegisterS,
        "SP" => Source8::RegisterSP,
        v => panic!("unknown register type '{:?}'.", v),
    }
}

fn parse_memory(node: &Pair<Rule>) -> Source8 {
    let addr = node.as_str()[3..].to_owned();
    let bytes = hex::decode(addr).unwrap();
    let mut addr: usize = 0;

    for byte in bytes.iter() {
        addr = addr << 8 | (*byte as usize);
    }

    Source8::Memory(addr)
}

fn parse_value8(node: &Pair<Rule>) -> u8 {
    let hexa = node.as_str()[2..].to_owned();
    let val = hex::decode(hexa).unwrap();

    val[0] as u8
}
