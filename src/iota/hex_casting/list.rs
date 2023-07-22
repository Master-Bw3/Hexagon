use std::{collections::HashMap, ops::Not, rc::Rc};

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
}
