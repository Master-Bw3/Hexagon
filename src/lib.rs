use std::fs;

use crate::interpreter::interpret;
mod compiler;
mod interpreter;
mod iota;
mod parser;
mod pattern_registry;
mod patterns;
pub mod parse_config;

use interpreter::{state::State, mishap::Mishap};
use parse_config::{parse_config, Config};
use pattern_registry::{PatternRegistry, PatternRegistryExt};

type RunResult = Result<State, (Mishap, (usize, usize))>;

pub fn run(source: &String, opt_config: &Option<&Config>) -> RunResult {


    let great_spell_sigs = if let Some(conf) = &opt_config.as_ref() {
        conf.great_spell_sigs.clone()
    } else {
        PatternRegistry::gen_default_great_sigs()
    };

    let parse_result = parser::parse(&source, &great_spell_sigs).unwrap();
    
    interpret(parse_result, opt_config)
}
