use std::rc::Rc;

use reqwest::Error;

use crate::{compiler::nbt::compile_nbt, iota::Iota};

pub fn send_hex(iota_list: Vec<Rc<dyn Iota>>) -> Result<(), Error> {
    let client = reqwest::blocking::Client::new();
    let content = format!("SNBT={}", compile_nbt(iota_list));
    let response = client.post("http://localhost:9000/hexPost").body(content).send()?.text()?;
    println!("{response}");

    Ok(())
}
