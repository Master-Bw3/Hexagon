use crate::{
    interpreter::{mishap::Mishap, state::State},
    iota::{EntityIota, Iota},
};

pub fn get_caster(state: State) -> Result<State, Mishap> {
    let caster = Iota::Entity(EntityIota {
        name: "caster".to_string(),
        entity_type: "minecraft:player".to_string(),
    });

    let mut new_stack = state.stack.clone();
    new_stack.push(caster);

    Ok(State {
        stack: new_stack,
        ..state
    })
}

