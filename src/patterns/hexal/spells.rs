use std::{rc::Rc, thread::spawn};

use im::{vector, Vector};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Entity, EntityType, Holding, StackExt, State, Wisp}, continuation::iota_list_to_ast_node_list,
    },
    iota::{hex_casting::{
        list::ListIota, number::NumberIota, pattern::PatternIota, vector::VectorIota, entity::EntityIota,
    }, self, Iota},
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

    let code_list: Vector<Rc<dyn Iota>> = match code {
        crate::interpreter::state::Either::L(pat) => {
            let mut vec: Vector<Rc<dyn Iota>> = vector![];
            vec.push_back(pat);
            vec
        },
        crate::interpreter::state::Either::R(list) => (*list).clone(),
    };

    let wisp: Rc<dyn Iota> = Rc::new(EntityIota{ name: Rc::from(format!("Wisp #{}", num_wisps + 1).as_str()), uuid: String::new() });

    state.wisps.push_back(Wisp {
        code: iota_list_to_ast_node_list(Rc::new(code_list)),
        stack: vector![],
        ..Default::default()
    });

    Ok(state)
}
