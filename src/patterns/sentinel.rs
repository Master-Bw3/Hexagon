use std::ops::Sub;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{Iota, NullIota},
    pattern_registry::PatternRegistry,
};

pub fn create<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_vector(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    state.sentinal_location = Some(iota);

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
    let operation_result = match state.sentinal_location {
        Some(vec) => Iota::Vector(vec),
        None => Iota::Null(NullIota::Null),
    };

    state.stack.push(operation_result);
    Ok(state)
}

pub fn wayfind<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_vector(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operation_result = match state.sentinal_location {
        Some(vec) => Iota::Vector(vec.sub(iota).normalize()),
        None => Iota::Null(NullIota::Null),
    };

    state.stack.push(operation_result);
    Ok(state)
}
