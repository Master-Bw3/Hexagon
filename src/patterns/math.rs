use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::Iota,
    pattern_registry::PatternRegistry,
};

pub fn add<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_num_or_vec(0, arg_count)?,
        state.stack.get_num_or_vec(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = match iotas {
        (Either::L(num1), Either::L(num2)) => Iota::Number(num1 + num2),
        (Either::L(num), Either::R(vec)) => Iota::Vector(vec.add_scalar(num)),
        (Either::R(vec), Either::L(num)) => Iota::Vector(vec.add_scalar(num)),
        (Either::R(vec1), Either::R(vec2)) => Iota::Vector(vec1 + vec2),
    };

    state.stack.push(operation_result);

    Ok(state)
}

pub fn subtract<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_num_or_vec(0, arg_count)?,
        state.stack.get_num_or_vec(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operation_result = match iotas {
        (Either::L(num1), Either::L(num2)) => Iota::Number(num1 - num2),
        (Either::L(num), Either::R(vec)) => Iota::Vector((-vec).add_scalar(num)),
        (Either::R(vec), Either::L(num)) => Iota::Vector(vec.add_scalar(-num)),
        (Either::R(vec1), Either::R(vec2)) => Iota::Vector(vec1 - vec2),
    };

    state.stack.push(operation_result);

    Ok(state)
}