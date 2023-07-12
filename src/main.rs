use std::{env, fs};

use hexagon::parse_config::parse_config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let source_path = &args.get(1).expect("Expected File Path");

    let default_config_path = "config.toml".to_string();
    let config_path = args.get(1).unwrap_or(&default_config_path);

    let config = fs::read_to_string(config_path).map(parse_config).ok();

    let source = fs::read_to_string(source_path).expect("Should have been able to read the file");

    let interpreter_result = hexagon::run(&source, &config.as_ref());

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
