use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::state::{Entity, EntityType},
    iota::Iota,
};

#[derive(Clone, Debug)]
pub struct EntityIota {
    name: Rc<str>,
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
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<EntityIota>() {
            Some(other) => self.name == other.name,
            None => false,
        }
    }
}
