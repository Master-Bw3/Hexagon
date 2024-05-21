use std::rc::Rc;

use owo_colors::OwoColorize;

use crate::{iota::Iota, parser::Location};

use super::mishap::Mishap;

pub fn print_interpreter_error(
    (err, location, caused_by): (Mishap, Location, String),
    source: &str,
    source_path: &str,
) {
    match location {
        Location::Unknown => {
            print_err_msg(&err, &String::new());
            print_mishap_hint(&err, &String::new());
        }
        Location::Line(line, col) => {
            let location = format!("{source_path}:{line}:{col}");
            let line_content = source.lines().collect::<Vec<_>>()[line - 1];
            let pad_len = line.to_string().len();
            let padding = vec![" "; pad_len].concat();

            print_err_msg(&err, &padding);
            print_location(&location, &padding);
            eprintln!(" {padding} {}", "|".magenta().bold());
            print_mishap_content(line, line_content, &padding);

            print_mishap_hint(&err, &padding);
        }
        Location::List(_) => {
            print_err_msg(&err, &String::new());
            print_mishap_hint(&err, &String::new());
        },
    }
}

fn print_err_msg(err: &Mishap, padding: &String) {
    let error_label = "Error:".red().bold().to_string();
    let error_msg = err.error_message().bold().to_string();

    eprintln!("{error_label} {error_msg}");
}

fn print_location(location: &String, padding: &String) {
    eprintln!(" {padding} {} {location}", "@".magenta().bold());
}

fn print_mishap_hint(err: &Mishap, padding: &String) {
    let hint_label = "Hint:".yellow().bold().to_string();

    if let Some(hint) = err.error_hint() {
        eprintln!(" {padding} {} {hint_label} {hint}", "+".magenta().bold(),);
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

//tf is this atrocity
fn print_eval_mishap_content(pat_list: &[Rc<dyn Iota>], err_index: usize, pad_len: usize) {
    let err_pad_len = err_index.to_string().len();
    let padding = vec![" "; pad_len].concat();
    let extra_padding = vec![" "; pad_len - err_pad_len].concat();

    let context_pre: Vec<_> = if pat_list[..err_index].len() >= 3 {
        pat_list[(err_index - 3)..err_index].to_owned()
    } else {
        pat_list[..err_index].to_owned()
    }
    .iter()
    .map(|iota: &Rc<dyn Iota>| iota.display())
    .collect();

    let context_post: Vec<_> = if pat_list[err_index..].len() > 3 {
        pat_list[(err_index + 1)..=err_index + 3].to_owned()
    } else {
        pat_list[(err_index + 1)..].to_owned()
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
