use std::fs;
pub mod parser;

pub fn run() {
    let source = fs::read_to_string("test.txt").expect("Should have been able to read the file");

    let parse_result = parser::parse(&source);

    println!("{:#?}", parse_result.unwrap())
}
