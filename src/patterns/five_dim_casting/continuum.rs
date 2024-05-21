use std::{cell::RefCell, rc::Rc};

use im::vector;

use crate::{
    interpreter::{
        continuation::{iota_list_to_ast_node_list, ContinuationFrame, FrameIterate},
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{
        five_dim_casting::continuum::ContinuumIota,
        hex_casting::{
            list::ListIota,
            number::{NumberIota, NumberIotaExt},
            pattern::{PatternIota, SignatureExt},
        },
        Iota,
    },
    parser::{ActionValue, AstNode, Location},
    pattern_registry::PatternRegistry,
};

pub fn number_stream<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let one: Rc<dyn Iota> = Rc::new(
        PatternIota::from_name(
            pattern_registry,
            "number",
            Some(ActionValue::Iota(Rc::new(1.0))),
            Location::Unknown,
        )
        .unwrap(),
    );
    let add: Rc<dyn Iota> =
        Rc::new(PatternIota::from_name(pattern_registry, "add", None, Location::Unknown).unwrap());

    let continuum = ContinuumIota {
        front_val: Rc::new(0.0),
        gen_next_func: iota_list_to_ast_node_list(Rc::new(vector![one, add])),
        maps: vector![],
    };

    state.stack.push_back(Rc::new(continuum));

    Ok(state)
}

pub fn get<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    {
        let arg_count = 2;
        let iotas = (
            (*state.stack.get_iota::<ContinuumIota>(0, arg_count)?).clone(),
            state.stack.get_iota::<NumberIota>(1, arg_count)?,
        );
        state.stack.remove_args(&arg_count);

        state
            .continuation
            .push_back(ContinuationFrame::Iterate(FrameIterate {
                base_stack: None,
                index: 0,
                collect: (*iotas.1 as usize, *iotas.1 as usize),
                acc: Rc::new(RefCell::new(vector![])),
                initial_iota: iotas.0.front_val,
                gen_next_code: iotas.0.gen_next_func.clone(),
                maps: iotas.0.maps,
                collect_single: true,
            }));

        Ok(state)
    }
}

pub fn slice<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let continuum = (*state.stack.get_iota::<ContinuumIota>(0, arg_count)?).clone();
    let iotas = (
        (state.stack)
            .get_iota::<NumberIota>(1, arg_count)?
            .positive_int(1)? as usize,
        (state.stack)
            .get_iota::<NumberIota>(2, arg_count)?
            .positive_int(2)? as usize,
    );
    state.stack.remove_args(&arg_count);
    if iotas.0 >= iotas.1 {
        state.stack.push_back(Rc::new(vector![]))
    } else {
        state
            .continuation
            .push_back(ContinuationFrame::Iterate(FrameIterate {
                base_stack: None,
                index: 0,
                collect: (iotas.0, iotas.1 - 1),
                acc: Rc::new(RefCell::new(vector![])),
                initial_iota: continuum.front_val,
                gen_next_code: continuum.gen_next_func.clone(),
                maps: continuum.maps,
                collect_single: false,
            }));
    }

    Ok(state)
}

pub fn map<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut continuum = (*state.stack.get_iota::<ContinuumIota>(0, arg_count)?).clone();
    let code = state
        .stack
        .get_iota_a_or_b::<PatternIota, ListIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    match code {
        crate::interpreter::state::Either::L(pattern) => {
            continuum.maps.push_back(vector![AstNode::Action {
                location: Location::Unknown,
                name: pattern.signature.as_str(),
                value: *pattern.value.clone(),
            }])
        }

        crate::interpreter::state::Either::R(pattern_list) => continuum
            .maps
            .push_back(iota_list_to_ast_node_list(pattern_list)),
    }

    state.stack.push_back(Rc::new(continuum));

    Ok(state)
}

pub fn make_stream<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let front_val = state.stack.get_any_iota(0, arg_count)?.clone();
    let code = state
        .stack
        .get_iota_a_or_b::<PatternIota, ListIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    let gen_next_func = match code {
        crate::interpreter::state::Either::L(pattern) => {
            vector![AstNode::Action {
                location: Location::Unknown,
                name: pattern.signature.as_str(),
                value: *pattern.value.clone(),
            }]
        }
        crate::interpreter::state::Either::R(pattern_list) => {
            iota_list_to_ast_node_list(pattern_list)
        }
    };

    let continuum = ContinuumIota {
        front_val,
        gen_next_func,
        maps: vector![],
    };

    state.stack.push_back(Rc::new(continuum));

    Ok(state)
}

pub fn deconstruct<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let continuum = (*state.stack.get_iota::<ContinuumIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    state
        .continuation
        .push_back(ContinuationFrame::Iterate(FrameIterate {
            base_stack: None,
            index: 0,
            collect: (1, 1),
            acc: Rc::new(RefCell::new(vector![])),
            initial_iota: continuum.front_val.clone(),
            gen_next_code: continuum.gen_next_func.clone(),
            maps: vector![],
            collect_single: true,
        }));

    state
        .continuation
        .push_back(ContinuationFrame::Iterate(FrameIterate {
            base_stack: None,
            index: 0,
            collect: (0, 0),
            acc: Rc::new(RefCell::new(vector![])),
            initial_iota: continuum.front_val,
            gen_next_code: continuum.gen_next_func.clone(),
            maps: continuum.maps,
            collect_single: true,
        }));

    Ok(state)
}
