use std::rc::Rc;

use im::{vector, Vector};

use crate::{
    interpreter::{
        continuation::iota_list_to_ast_node_list,
        mishap::Mishap,
        state::{Entity, EntityType, Holding, StackExt, State, Wisp},
    },
    iota::{
        hex_casting::{
            entity::EntityIota, list::ListIota, number::NumberIota, pattern::PatternIota,
            vector::VectorIota,
        },
        Iota,
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
                iota.clone()
                    .downcast_rc::<VectorIota>()
                    .map_err(|_| Mishap::IncorrectIota {
                        index: 0,
                        expected: "List of vectors".to_string(),
                        received: iota.clone(),
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
    let _pos = state.stack.get_iota::<VectorIota>(1, arg_count)?;
    let _battery = state.stack.get_iota::<NumberIota>(2, arg_count)?;
    state.stack.remove_args(&arg_count);

    let num_wisps = {
        state
            .entities
            .keys()
            .filter(|k| k.starts_with("Wisp"))
            .count()
    };

    let wisp_name = format!("Wisp #{}", num_wisps + 1);

    state.entities.insert(
        wisp_name.clone(),
        Entity {
            name: wisp_name.clone(),
            uuid: String::new(),
            entity_type: EntityType::Wisp,
            holding: Box::new(Holding::None),
        },
    );

    let code_list: Vector<Rc<dyn Iota>> = match code {
        crate::interpreter::state::Either::L(pat) => {
            let mut vec: Vector<Rc<dyn Iota>> = vector![];
            vec.push_back(pat);
            vec
        }
        crate::interpreter::state::Either::R(list) => (*list).clone(),
    };

    let wisp = EntityIota {
        name: Rc::from(wisp_name.clone().as_str()),
        uuid: String::new(),
    };

    state.wisps.insert(
        wisp_name,
        Wisp {
            code: iota_list_to_ast_node_list(Rc::new(code_list)),
            stack: vector![],
            self_ref: Some(wisp),
            ..Default::default()
        },
    );

    Ok(state)
}
