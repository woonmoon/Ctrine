#[macro_use]
extern crate lalrpop_util;

use std::env;
use std::fs;

lalrpop_mod!(pub parser);
pub mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    // &args[0] is "target/debug/chocolat"
    let filename = &args[1];
    let program = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("prog {:?}", program);
    let ast = parser::DeclarationParser::new().parse(&program).unwrap();
    // println!("Got program \n {}", program);
    println!("ast {:?}", ast);
}
