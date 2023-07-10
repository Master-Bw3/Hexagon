use std::fs;

use crate::interpreter::interpret;
mod compiler;
mod interpreter;
mod iota;
mod parse_config;
mod parser;
mod pattern_registry;
mod patterns;

use parse_config::parse_config;

pub fn run() {
    let path = "./test.hexcasting";
    let source = fs::read_to_string(path).expect("Should have been able to read the file");

    let parse_result = parser::parse(&source).unwrap();

    let config_path = "./test.toml";
    let config_source = fs::read_to_string(path).expect("Should have been able to read the file");

    let config = parse_config(config_source);

    let interpreter_result = interpret(parse_result, Some(config));

    match interpreter_result {
        Ok(result) => println!("\n result: {:?} \n {:?}", result.stack, result.buffer),
        Err((err, (line, col))) => {
            eprintln!("\x1b[31mError:\x1b[0m {:?}, {path}:{line}:{col}", err)
        }
    }
}
