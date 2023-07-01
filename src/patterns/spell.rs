use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State, Stack},
    },
    iota::Iota,
};


pub fn spell<'a>(arg_count: &'a usize) -> Box<(dyn Fn(&mut State) -> Result<&mut State, Mishap> + 'a)> {
    Box::new(move |state: &mut State| {
        state.stack.remove_args(arg_count);
        Ok(state)
    })
}
