use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Stack, StackExt, State},
    },
    iota::{Iota, NumberIota},
    pattern_registry::PatternRegistry,
};

type ActionType = dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>;

type GetterType = fn(&Stack, usize, usize) -> Result<NumberIota, Mishap>;

pub fn spell(arg_count: usize, getters: Vec<GetterType>) -> Box<ActionType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| {
            for (index, getter) in getters.iter().enumerate() {
                getter(&state.stack, index, arg_count)?;
            }
            state.stack.remove_args(&arg_count);
            Ok(state)
        },
    )
}

pub fn push_const(iota: Iota) -> Box<ActionType> {
    Box::new(
        move |state: &mut State, _: &PatternRegistry| -> Result<&mut State, Mishap> {
            state.stack.push(iota.clone());
            Ok(state)
        },
    )
}
