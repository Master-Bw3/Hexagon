use crate::iota::Iota;

pub fn get_caster() -> Vec<Iota> {
    vec![Iota::Entity {
        name: "caster",
        entity_type: "minecraft:player",
    }]
}
