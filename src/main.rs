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

pub struct LogicalDecoder {}

impl LogicalDecoder {
    pub fn new() -> LogicalDecoder {
        LogicalDecoder {}
    }

    pub fn solve(&self) -> bool {
        true
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

struct StopCondition {
     solver: Box<dyn Fn(val: i32) -> bool>,
}

/*
impl StopCondition {
    pub fn new(pair: Pair) -> StopCondition {

    }
}
*/
