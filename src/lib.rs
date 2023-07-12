use std::{fs, env};

use crate::interpreter::interpret;
mod compiler;
mod interpreter;
mod iota;
mod parser;
mod pattern_registry;
mod patterns;
mod parse_config;

use interpreter::{state::State, mishap::Mishap};
use parse_config::{parse_config, Config};
use pattern_registry::{PatternRegistry, PatternRegistryExt};

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let source_path = &args.get(1).expect("Expected File Path");

    let default_config_path = "config.toml".to_string();
    let config_path = args.get(2).unwrap_or(&default_config_path);

    let config = fs::read_to_string(config_path).map(parse_config).ok();

    let source = fs::read_to_string(source_path).expect("Should have been able to read the file");


    let great_spell_sigs = if let Some(conf) = &config.as_ref() {
        conf.great_spell_sigs.clone()
    } else {
        PatternRegistry::gen_default_great_sigs()
    };

    let parse_result = parser::parse(&source, &great_spell_sigs).unwrap();
    
    

    let interpreter_result = interpret(parse_result, &config.as_ref());

    match interpreter_result {
        Ok(result) => println!("\n result: {:?} \n {:?}", result.stack, result.buffer),
        Err((err, (line, col))) => {
            eprintln!(
                "\x1b[31mError:\x1b[0m {:?}, {source_path}:{line}:{col}",
                err
            )
        }
    };
}
