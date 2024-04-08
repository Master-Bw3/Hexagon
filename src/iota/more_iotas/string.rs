use std::rc::Rc;

use im::Vector;
use serde_json::Map;

use crate::iota::Iota;

pub type StringIota = String;

impl Iota for StringIota {
    fn display(&self) -> String {
        format!("{:?}", self)
    }

    fn display_type_name() -> String {
        "String".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<StringIota>() {
            Some(other) => self == other,
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        format!("{{\"hexcasting:type\": \"moreiotas:string\", \"hexcasting:data\": \"{self}\"}}")
    }
    
    fn serialize_to_json(&self) -> serde_json::Value {
        let mut map = Map::new();
        map.insert("iotaType".to_string(), serde_json::Value::String("string".to_string()));
        map.insert("value".to_string(), serde_json::Value::String(self.clone()));

        serde_json::Value::Object(map)
    }
}

pub trait StringVecExt {
    fn is_string_vec(&self) -> bool;
}

impl StringVecExt for Vector<Rc<dyn Iota>> {
    fn is_string_vec(&self) -> bool {
        self.iter()
            .filter(|i| (**i).clone().downcast_rc::<StringIota>().is_ok())
            .count()
            == self.len()
    }
}
