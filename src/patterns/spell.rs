use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State, Stack},
    },
    iota::Iota,
};


fn spell(arg_count: &'static usize) -> &'static dyn Fn(&mut State) -> Result<&mut State, Mishap> {
    &(|state| {
        state.stack.remove_args(arg_count);
        Ok(state)
    })
}
