use std::rc::Rc;

use crate::{compiler::nbt::compile_nbt, iota::Iota};

pub fn send_hex(iota_list: Vec<Rc<dyn Iota>>, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let content = format!("SNBT={}", compile_nbt(iota_list));
    let response = client.post(url).body(content).send()?.text()?;

    Ok(response)
}