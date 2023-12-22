use compiler::{compile_to_iotas, nbt::gen_give_cmd};

use hex_server::send_hex;
use interpreter::error::print_interpreter_error;
use iota::Iota;

use owo_colors::OwoColorize;
use std::{collections::HashMap, env, fs};

use crate::interpreter::interpret;
pub mod compiler;
pub mod interpreter;
pub mod iota;
pub mod parse_config;
pub mod parser;
pub mod pattern_registry;
pub mod patterns;
pub mod hex_server;

use parse_config::{parse_config, Config};
use pattern_registry::{PatternRegistry, PatternRegistryExt};

struct Args {
    command: Command,
    source_path: String,
    config_path: String,
}

impl Args {
    fn get() -> Args {
        let args: Vec<String> = env::args().collect();

        let command = args.get(1).expect("Expected command");

        let source_path = args.get(2).expect("Expected File Path").to_owned();

        let default_config_path = "config.toml".to_string();
        let config_path = args.get(3).unwrap_or(&default_config_path).to_owned();

        Args {
            command: Args::get_cmd(command),
            source_path,
            config_path,
        }
    }

    fn get_cmd(cmd: &str) -> Command {
        match cmd {
            "run" => Command::Run,
            "build" => Command::Build,
            "run_external" => Command::RunExternal,
            _ => panic!("invalid command"),
        }
    }
}

enum Command {
    Run,
    Build,
    RunExternal,
}

pub fn run() {
    let args = Args::get();

    let mut config = fs::read_to_string(args.config_path)
        .map(parse_config)
        .unwrap_or_else(|_| Config {
            libraries: HashMap::new(),
            entities: HashMap::new(),
            great_spell_sigs: PatternRegistry::gen_default_great_sigs(),
        });

    let source =
        fs::read_to_string(&args.source_path).expect("Should have been able to read the file");

    let parse_result = parser::parse(&source, &config.great_spell_sigs, &mut config.entities);
    let (ast, macros) = match parse_result {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}\n{}", "Parsing Error:".red().bold(), err);
            return;
        }
    };

    if let Command::Run = args.command {
        let interpreter_result = interpret(ast, &config, macros, &source, &args.source_path);

        match interpreter_result {
            Ok(result) => println!(
                "\nresult: {} \n {:?}",
                result.stack.display(),
                result.buffer
            ),
            Err(err) => {
                print_interpreter_error(err, &source, &args.source_path);
            }
        };

    } else if let Command::Build = args.command {
        let pattern_registry = PatternRegistry::construct(&config.great_spell_sigs);
        let compile_result = compile_to_iotas(&ast, None, &pattern_registry, &macros);
        match compile_result {
            // Ok(result) => println!("\nresult: {}", Vector::from(result).display()),
            Ok(result) => println!("\nresult: {}", gen_give_cmd(result)),

            Err(err) => {
                print_interpreter_error(err, &source, &args.source_path);
            }
        };
    } else if let Command::RunExternal = args.command { 
        let pattern_registry = PatternRegistry::construct(&config.great_spell_sigs);
        let compile_result = compile_to_iotas(&ast, None, &pattern_registry, &macros);
        match compile_result {
            // Ok(result) => println!("\nresult: {}", Vector::from(result).display()),
            Ok(result) => {
                send_hex(result).unwrap();
                println!("hex sent to server")
            },

            Err(err) => {
                print_interpreter_error(err, &source, &args.source_path);
            }
        };
    }

}
