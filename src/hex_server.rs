use std::rc::Rc;

use reqwest::header::CONTENT_TYPE;

use crate::{compiler::nbt::compile_nbt, iota::Iota};

pub fn send_hex(iota_list: Vec<Rc<dyn Iota>>, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let content = [("SNBT", compile_nbt(iota_list))];
    let response = client.post(url).form(&content).send()?.text()?;

    Ok(response)
}