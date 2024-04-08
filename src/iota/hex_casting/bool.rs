use serde::Serialize;
use serde_json::Map;

use crate::iota::Iota;

pub type BooleanIota = bool;

impl Iota for BooleanIota {
    fn display(&self) -> String {
        self.to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<BooleanIota>() {
            Some(other) => other == self,
            None => false,
        }
    }

    fn display_type_name() -> String {
        "Boolean".to_string()
    }

    fn serialize_to_nbt(&self) -> String {
        let byte = if *self { "1b" } else { "0b" };

        format!("{{\"hexcasting:type\": \"hexcasting:boolean\", \"hexcasting:data\": {byte}}}")
    }
    
    fn serialize_to_json(&self) -> serde_json::Value {
        let mut map = Map::new();
        map.insert("iotaType".to_string(), serde_json::Value::String("boolean".to_string()));
        map.insert("value".to_string(), serde_json::Value::Bool(*self));

        serde_json::Value::Object(map)  
      }
}