use crate::{
    interpreter::{
        mishap::Mishap,
        stack::{NumOrVec, StackExt, State},
    },
    iota::Iota,
};

pub fn add(state: State) -> Result<State, Mishap> {
    let iotas = (
        state.stack.get_num_or_vec(0)?,
        state.stack.get_num_or_vec(0)?,
    );

    let operation_result = match iotas {
        (NumOrVec::Num(num1), NumOrVec::Num(num2)) => Iota::Number(num1 + num2),
        (NumOrVec::Num(num), NumOrVec::Vec(vec)) => Iota::Vector(vec.add_scalar(num)),
        (NumOrVec::Vec(vec), NumOrVec::Num(num)) => Iota::Vector(vec.add_scalar(num)),
        (NumOrVec::Vec(vec1), NumOrVec::Vec(vec2)) => 
        Iota::Vector(vec1 + vec2),
    };

    let mut new_stack = state.stack.clone();
    new_stack.push(operation_result);

    Ok(State {
        stack: new_stack,
        ..state
    })
}
