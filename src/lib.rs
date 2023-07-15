use interpreter::mishap::Mishap;
use iota::Iota;
use owo_colors::{colors::Red, OwoColorize};
use std::{collections::HashMap, env, fmt::format, fs};

use crate::{interpreter::interpret, patterns::Pattern};
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

fn print_interpreter_error(
    (err, (line, col)): (Mishap, (usize, usize)),
    source: &str,
    source_path: &str,
) {
    let location = format!("{source_path}:{line}:{col}");
    let line_content = source.lines().collect::<Vec<_>>()[line - 1];
    let pad_len = line.to_string().len();
    let padding = vec![" "; pad_len].concat();

    print_err_msg(&err, &padding, &location);
    eprintln!(" {padding} {}", "|".magenta().bold());
    match err {
        Mishap::EvalMishap(ref stack, index, _) => print_eval_mishap_content(stack, index, pad_len),
        _ => print_mishap_content(line, line_content, &padding),
    }
    print_mishap_hint(&err, &padding);
}

fn print_err_msg(err: &Mishap, padding: &String, location: &String) {
    let error_label = "Error:".red().bold().to_string();
    let error_msg = err.error_message().bold().to_string();

    eprintln!("{error_label} {error_msg}");
    eprintln!(" {padding} {} {location}", "@".magenta().bold());
}

fn print_mishap_hint(err: &Mishap, padding: &String) {
    let hint_label = "Hint:".yellow().bold().to_string();

    match err.error_hint() {
        Some(hint) => {
            eprintln!(" {padding} {} {hint_label} {hint}", "+".magenta().bold(),);
        }
        None => (),
    }
}

fn print_mishap_content(line: usize, line_content: &str, padding: &String) {
    eprintln!(
        " {} {} {line_content}",
        line.magenta().bold(),
        ">".magenta().bold()
    );
    eprintln!(" {padding} {}", "|".magenta().bold());
}
fn print_eval_mishap_content(pat_list: &Vec<Iota>, err_index: usize, pad_len: usize) {
    let err_pad_len = err_index.to_string().len();
    let padding = vec![" "; pad_len].concat();
    let extra_padding = vec![" "; (pad_len - err_pad_len)].concat();

    let context_pre: Vec<_> = if pat_list[..err_index].len() >= 3 {
        pat_list[(err_index - 3)..err_index].to_vec()
    } else {
        pat_list[..err_index].to_vec()
    }
    .iter()
    .map(|iota| iota.display())
    .collect();

    let context_post: Vec<_> = if pat_list[err_index..].len() > 3 {
        pat_list[((err_index + 1)..=err_index + 3)].to_vec()
    } else {
        pat_list[(err_index + 1)..].to_vec()
    }
    .iter()
    .map(|iota| iota.display())
    .collect();

    let action = &pat_list[err_index];

    for content in &context_pre {
        eprintln!(" {padding} {} {content}", "|".magenta().bold());
    }

    eprintln!(
        "{extra_padding} {} {} {}",
        err_index.magenta().bold(),
        ">".magenta().bold(),
        action.display().bold()
    );

    for content in &context_post {
        eprintln!(" {padding} {} {content}", "|".magenta().bold());
    }

    eprintln!(" {padding} {}", "|".magenta().bold());

    let note_label = "Note:".yellow().bold().to_string();
    eprintln!(" {padding} {} {note_label} This error originated from either Hermes' Gambit or Thoth's Gambit. Above is the list that was evaluated and the iota that caused the mishap", "+".magenta().bold(),);
}
