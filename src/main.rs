use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let source_path = &args.get(1).expect("Expected File Path");
    let config_path = &args.get(2);

    hexagon::run(source_path, config_path)
}
