use std::{rc::Rc, thread::spawn};

use im::vector;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Entity, EntityType, Holding, StackExt, State},
    },
    iota::hex_casting::{
        list::ListIota, number::NumberIota, pattern::PatternIota, vector::VectorIota,
    },
    pattern_registry::PatternRegistry,
};

pub fn particles<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let arg = state
        .stack
        .get_iota_a_or_b::<VectorIota, ListIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    match arg {
        crate::interpreter::state::Either::L(_) => (),
        crate::interpreter::state::Either::R(list) => {
            for iota in (*list).clone() {
                iota.clone().downcast_rc::<VectorIota>().map_err(|_| {
                    Mishap::IncorrectIota(0, "List of vectors".to_string(), iota.clone())
                })?;
            }
        }
    }

    Ok(state)
}

pub fn summon_wisp_ticking<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let code = state
        .stack
        .get_iota_a_or_b::<PatternIota, ListIota>(0, arg_count)?;
    let pos = state.stack.get_iota::<VectorIota>(1, arg_count)?;
    let battery = state.stack.get_iota::<NumberIota>(2, arg_count)?;
    state.stack.remove_args(&arg_count);

    let num_wisps = {
        state
            .entities
            .keys()
            .filter(|k| k.starts_with("Wisp"))
            .count()
    };

    state.entities.insert(
        format!("Wisp #{}", num_wisps + 1),
        Entity {
            name: format!("Wisp #{}", num_wisps + 1),
            uuid: String::new(),
            entity_type: EntityType::Wisp,
            holding: Box::new(Holding::None),
        },
    );



    Ok(state)
}
