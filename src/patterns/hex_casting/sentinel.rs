use std::{ops::Sub, rc::Rc};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{Iota, hex_casting::{null::NullIota, vector::VectorIota}},
    pattern_registry::PatternRegistry,
};

pub fn create<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota::<VectorIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    state.sentinal_location = Some(*iota);

    Ok(state)
}

pub fn destroy<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    state.sentinal_location = None;

    Ok(state)
}

pub fn get_pos<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let operation_result: Rc<dyn Iota> = match state.sentinal_location {
        Some(vec) => Rc::new(vec),
        None => Rc::new(NullIota),
    };

    state.stack.push_back(operation_result);
    Ok(state)
}

pub fn wayfind<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = *state.stack.get_iota::<VectorIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result: Rc<dyn Iota> = match state.sentinal_location {
        Some(vec) => Rc::new(vec.sub(iota).normalize()),
        None => Rc::new(NullIota),
    };

    state.stack.push_back(operation_result);
    Ok(state)
}
