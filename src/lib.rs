use interpreter::mishap::Mishap;
use owo_colors::{colors::Red, OwoColorize};
use std::{collections::HashMap, env, fmt::format, fs};

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
            _ => panic!("invalid command"),
        }
    }
}

enum Command {
    Run,
    Build,
}

pub fn run() {
    let args = Args::get();

    let config = fs::read_to_string(args.config_path).map(parse_config).ok();

    let source =
        fs::read_to_string(&args.source_path).expect("Should have been able to read the file");

    let great_spell_sigs = if let Some(conf) = &config.as_ref() {
        conf.great_spell_sigs.clone()
    } else {
        PatternRegistry::gen_default_great_sigs()
    };

    let mut entities = config
        .as_ref()
        .map(|conf| conf.entities.clone())
        .unwrap_or(HashMap::new());

    let parse_result = parser::parse(&source, &great_spell_sigs, &mut entities).unwrap();

    if let Command::Run = args.command {
        let interpreter_result = interpret(parse_result, &config.as_ref(), entities);

        match interpreter_result {
            Ok(result) => println!("\n result: {:?} \n {:?}", result.stack, result.buffer),
            Err(err) => {
                print_interpreter_error(err, &source, &args.source_path);
            }
        };
    } else {
        todo!("build")
    }
}

fn print_interpreter_error((err, (line, col)): (Mishap, (usize, usize)), source: &str, source_path: &str) {
    let error_label = "Error:".red().bold().to_string();
    let error_msg = err.error_message().bold().to_string();
    let location = format!("{source_path}:{line}:{col}");
    let line_content = source.lines().collect::<Vec<_>>()[line - 1];
    let padding = vec![" "; line.to_string().len()].concat();
    let hint_label = "Hint:".yellow().bold().to_string();

    eprintln!("{error_label} {error_msg}");
    eprintln!(" {padding} {} {location}", "@".magenta().bold());
    eprintln!(" {padding} {}", "|".magenta().bold());
    eprintln!(
        " {} {} {line_content}",
        line.magenta().bold(),
        "|".magenta().bold()
    );
    eprintln!(" {padding} {}", "|".magenta().bold());
    match err.error_hint() {
        Some(hint) => {
                eprintln!(" {padding} {} {hint_label} {hint}", ">".magenta().bold(), );

        },
        None => (),
    }
}
