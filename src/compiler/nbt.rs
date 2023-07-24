use std::rc::Rc;

use im::Vector;

use crate::iota::Iota;

pub fn compile_nbt(iota_list: Vec<Rc<dyn Iota>>) -> String {
    let out = Vector::from(iota_list).serialize_to_nbt();
    format!("{{data: {out}}}")
}

pub fn gen_give_cmd(iota_list: Vec<Rc<dyn Iota>>) -> String {
    let data = compile_nbt(iota_list);
    format!("/give @p hexcasting:focus{data} 1")
}