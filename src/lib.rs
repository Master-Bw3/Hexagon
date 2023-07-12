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
use pattern_registry::{PatternRegistry, PatternRegistryExt};

pub fn run(source_path: &String, opt_config_path: &Option<&String>) {
    let config = opt_config_path.map(|config_path| {
        let config_source =
            fs::read_to_string(config_path).expect("Should have been able to read the file");
        parse_config(config_source)
    });

    let source = fs::read_to_string(source_path).expect("Should have been able to read the file");

    let great_spell_sigs = if let Some(conf) = &config.as_ref() {
        conf.great_spell_sigs.clone()
    } else {
        PatternRegistry::gen_default_great_sigs()
    };

    let parse_result = parser::parse(&source, &great_spell_sigs).unwrap();
    let interpreter_result = interpret(parse_result, config);

    match interpreter_result {
        Ok(result) => println!("\n result: {:?} \n {:?}", result.stack, result.buffer),
        Err((err, (line, col))) => {
            eprintln!(
                "\x1b[31mError:\x1b[0m {:?}, {source_path}:{line}:{col}",
                err
            )
        }
    }
}
