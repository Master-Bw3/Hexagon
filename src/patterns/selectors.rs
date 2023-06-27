use crate::iota::{EntityIota, Iota};

pub fn get_caster() -> Vec<Iota> {
    vec![Iota::Entity(EntityIota {
        name: "caster".to_string(),
        entity_type: "minecraft:player".to_string(),
    })]
}
