use std::{collections::HashMap, rc::Rc};

use serde_json::Map;

use crate::{
    interpreter::state::{Entity, EntityType},
    iota::Iota,
};

#[derive(Clone, Debug)]
pub struct EntityIota {
    pub name: Rc<str>,
    pub uuid: String,
}

impl EntityIota {
    pub fn is_of_type(
        &self,
        entity_type: Option<&EntityType>,
        entities: &HashMap<String, Entity>,
    ) -> bool {
        match entities.get(&self.name[..]) {
            Some(entity) => match entity_type {
                Some(t) => entity.entity_type == *t,
                None => true,
            },
            None => false,
        }
    }
}

impl Iota for EntityIota {
    fn display(&self) -> String {
        format!("@{}", self.name)
    }

    fn display_type_name() -> String {
        "Entity".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<EntityIota>() {
            Some(other) => self.name == other.name,
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        format!("{{\"hexcasting:type\": \"hexcasting:entity\", \"hexcasting:data\": {{name: '{{\"text\":\"\"}}', uuid: {}}}}}", self.uuid)
    }
    
    fn serialize_to_json(&self) -> serde_json::Value {
        let mut map = Map::new();
        map.insert("iotaType".to_string(), serde_json::Value::String("entity".to_string()));
        map.insert("value".to_string(), serde_json::Value::String(self.uuid.clone()));

        serde_json::Value::Object(map)
    }
}
