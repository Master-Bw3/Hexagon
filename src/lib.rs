use std::fs;

use crate::interpreter::interpret;
mod parser;
mod patterns;
mod iota;
mod interpreter;
mod pattern_registry;
mod compiler;


pub fn run() {
    let source = fs::read_to_string("test.hexcasting").expect("Should have been able to read the file");

    let parse_result = parser::parse(&source).unwrap();

    let interpreter_result = interpret(parse_result);

    println!("\n result: {:?} \n {:?}", interpreter_result.as_ref().unwrap().stack, interpreter_result.as_ref().unwrap().buffer)
}
