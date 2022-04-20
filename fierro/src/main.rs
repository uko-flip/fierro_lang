#[macro_use]
extern crate lalrpop_util;

use std::fs;
use std::env;

mod ast;

lalrpop_mod!(pub fierra_grammar);

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = fs::read_to_string(args[1].clone()).unwrap();
    let value = fierra_grammar::BlockParser::new().parse(&data).unwrap().evaluate();
}
