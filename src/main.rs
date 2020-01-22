extern crate ansi_term;

use ansi_term::Colour;
use ansi_term::Style;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error as PestError;
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
            Source8::RegisterX   => registers.register_x,
            Source8::RegisterY   => registers.register_y,
            Source8::RegisterSP  => registers.status_register,
            Source8::RegisterS   => registers.stack_pointer,
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
            BooleanExpression::Equal(source, val)   => source.get_value(registers, memory) == *val,
            BooleanExpression::GreaterOrEqual(source, val)   => source.get_value(registers, memory) >= *val,
            BooleanExpression::StrictlyGreater(source, val)   => source.get_value(registers, memory) > *val,
            BooleanExpression::LesserOrEqual(source, val)   => source.get_value(registers, memory) <= *val,
            BooleanExpression::StrictlyLesser(source, val)   => source.get_value(registers, memory) < *val,
            BooleanExpression::Different(source, val)   => source.get_value(registers, memory) != *val,
            BooleanExpression::Value(val)    => *val,
        }
    }
}

fn main() {
    println!("{}", Colour::Green.paint("Welcome in Lepr 0.0.1"));
    let prompt = format!("{}", Colour::Yellow.paint(">> "));
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!(
                    "{:?}",
                    BEParser::parse(Rule::boolean_expression, line.as_str())
                );
            }
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

pub fn parse(rule: Pair, line: &str) -> BooleanExpression {
    match rule.inner[0].rule {
        "boolean" => BooleanExpression::Value(rule.inner[0].span.str == "true"),
        "operation" => parse_operation(rule.inner[0].inner),
        smt   => panic!("unknown node type '{}'.", smt),
    }
}

fn parse_operation(nodes: [Pair; 3]) -> BooleanExpression {
    let lh_rule_name = nodes[0].rule;
    let lh = match lh_rule_name {
        "register8" => parse_register(&nodes[0]),
        "memory"    => parse_memory(&nodes[0]),
    };
    let rh = parse_value8(&nodes[2].span.str);
    match nodes[1].span.str {
        "="     => BooleanExpression::Equal(lh, rh),
        ">="    => BooleanExpression::GreaterOrEqual(lh, rh),
        ">"     => BooleanExpression::StrictlyGreater(lh, rh),
        "<="    => BooleanExpression::LesserOrEqual(lh, rh),
        "<"     => BooleanExpression::StrictlyLesser(lh, rh),
        "!="    => BooleanExpression::Different(lh, rh),
        v       => panic!("unknown 8 bits provider {:?}", v),
    }
}

fn parse_register(node: &Pair) -> Source8 {
    match *node.span.str {
        "A" => Source8::Accumulator,
        "X" => Source8::RegisterX,
        "Y" => Source8::RegisterY,
        "S" => Source8::RegisterS,
        "SP" => Source8::RegisterSP,
        v   => panic!("unknown register type '{:?}'.", v),
    }
}

fn parse_memory(node: &Pair) -> Source8 {
    let bytes = hex::decode(node.span.str).unwrap();
    let mut addr:usize = 0;

    for byte in bytes.iter() {
        addr = addr << 8 | ( byte as usize);
    }

    Source8::Memory(addr)
}

fn parse_value8(node: &Pair) -> u8 {
    let val = hex::decode(node.span.str[2..]).unwrap();

    val[0] as u8
}
