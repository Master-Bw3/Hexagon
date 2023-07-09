use std::fs;

use crate::interpreter::interpret;
mod compiler;
mod interpreter;
mod iota;
mod parser;
mod pattern_registry;
mod patterns;
mod parse_config;

pub fn run() {
    let path = "./test.hexcasting";
    let source = fs::read_to_string(path).expect("Should have been able to read the file");

    let parse_result = parser::parse(&source).unwrap();

    let interpreter_result = interpret(parse_result);

    match interpreter_result {
        Ok(result) => println!("\n result: {:?} \n {:?}", result.stack, result.buffer),
        Err((err, (line, col))) => eprintln!("\x1b[31mError:\x1b[0m {:?}, {path}:{line}:{col}", err),
    }
}
