use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
};

type ActionType<'a> = Box<(dyn Fn(&mut State) -> Result<&mut State, Mishap> + 'a)>;

pub fn spell(arg_count: &usize) -> ActionType {
    Box::new(move |state: &mut State| {
        state.stack.remove_args(arg_count);
        Ok(state)
    })
}
