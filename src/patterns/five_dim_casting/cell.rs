use std::{cell::RefCell, rc::Rc};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::five_dim_casting::cell::CellIota,
    pattern_registry::PatternRegistry,
};

pub fn create<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let cell: Rc<CellIota> = Rc::new(RefCell::new(iota));

    state.stack.push_back(cell);

    Ok(state)
}

pub fn replace<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let cell_iota = state.stack.get_iota::<CellIota>(0, arg_count)?;
    let iota = state.stack.get_any_iota(1, arg_count)?;
    state.stack.remove_args(&1);

    (*cell_iota).replace(iota);

    Ok(state)
}

pub fn unwrap<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let cell_iota = state.stack.get_iota::<CellIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let iota = ((*cell_iota).clone()).into_inner();
    state.stack.push_back(iota);

    Ok(state)
}
