use std::{collections::HashMap, ops::Not, rc::Rc};

use serde_json::Map;

use crate::{
    interpreter::state::{Entity, EntityType},
    iota::Iota,
};

use super::entity::EntityIota;

pub type ListIota = im::Vector<Rc<dyn Iota>>;

pub trait ListIotaExt {
    fn is_entity_list(
        &self,
        entity_type: Option<&EntityType>,
        entities: &HashMap<String, Entity>,
    ) -> bool;
}

impl ListIotaExt for ListIota {
    fn is_entity_list(
        &self,
        entity_type: Option<&EntityType>,
        entities: &HashMap<String, Entity>,
    ) -> bool {
        self.iter()
            .filter(|i| {
                i.downcast_ref::<EntityIota>()
                    .map(|e| e.is_of_type(entity_type, entities))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>()
            .len()
            == self.len()
    }
}

impl Iota for ListIota {
    fn display(&self) -> String {
        format!(
            "[{}]",
            self.iter()
                .map(|iota| iota.display())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn display_type_name() -> String {
        "List".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<ListIota>() {
            Some(other) => {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .map(|(rhs, lhs)| (**rhs).tolerates_other(&**lhs))
                        .collect::<im::Vector<bool>>()
                        .contains(&false)
                        .not()
            }
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        let out = self
            .iter()
            .map(|x| x.serialize_to_nbt())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{\"hexcasting:type\": \"hexcasting:list\", \"hexcasting:data\": [{out}]}}")
    }
    
    fn serialize_to_json(&self) -> serde_json::Value {
        let iotas = self
            .iter()
            .map(|iota| iota.serialize_to_json())
            .collect::<Vec<_>>();

        let mut map = Map::new();
        map.insert("iotaType".to_string(), serde_json::Value::String("list".to_string()));
        map.insert("value".to_string(), serde_json::Value::Array(iotas));

        serde_json::Value::Object(map)
    }
}
