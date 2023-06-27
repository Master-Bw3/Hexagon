use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::Iota,
};

pub fn add(state: State) -> Result<State, Mishap> {
    let mut new_stack = state.stack.clone();
    let arg_count = 2;
    let iotas = (
        new_stack.get_num_or_vec(0, arg_count)?,
        new_stack.get_num_or_vec(1, arg_count)?,
    );
    new_stack.drain((new_stack.len() - arg_count)..);

    let operation_result = match iotas {
        (Either::A(num1), Either::A(num2)) => Iota::Number(num1 + num2),
        (Either::A(num), Either::B(vec)) => Iota::Vector(vec.add_scalar(num)),
        (Either::B(vec), Either::A(num)) => Iota::Vector(vec.add_scalar(num)),
        (Either::B(vec1), Either::B(vec2)) => Iota::Vector(vec1 + vec2),
    };

    new_stack.push(operation_result);

    Ok(State {
        stack: new_stack,
        ..state
    })
}
