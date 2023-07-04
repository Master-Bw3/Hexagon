use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{Iota, NullIota, VectorIota},
    pattern_registry::PatternRegistry,
};

pub fn read_local<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    state.stack.push(
        state
            .ravenmind
            .clone()
            .unwrap_or(Iota::Null(NullIota::Null)),
    );

    Ok(state)
}

pub fn write_local<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota(1, arg_count)?.clone();
    state.stack.remove_args(&arg_count);

    state.ravenmind = Some(iota);

    Ok(state)
}
