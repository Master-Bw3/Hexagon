use std::rc::Rc;

use crate::iota::Iota;

pub fn compile_nbt(iota_list: Vec<Rc<dyn Iota>>) -> String {
    let out = serialize_hex(&iota_list);
    // print!("{iota_list:?}");
    format!("{out}")
}

pub fn gen_give_cmd(iota_list: Vec<Rc<dyn Iota>>) -> String {
    let data = compile_nbt(iota_list);
    format!("/give @p hexcasting:focus{{data: {data}}} 1")
}

fn serialize_hex(iota_list: &Vec<Rc<dyn Iota>>) -> String {
    let iotas_str = iota_list.iter()
        .map(|iota| iota.serialize_to_nbt()).collect::<Vec<_>>()
        .join(", ");
    
    return format!("{{hex: [{}]}}", iotas_str);
}
