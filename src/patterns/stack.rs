use crate::{
    interpreter::{
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{Iota, VectorIota},
    pattern_registry::PatternRegistry,
};

pub fn duplicate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota(0, arg_count)?.clone();
    state.stack.remove_args(&arg_count);

    state.stack.push(iota);

    Ok(state)
}

pub fn duplicate_n<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_number(1, arg_count)?.round() as usize,
    );
    state.stack.remove_args(&arg_count);

    state.stack.append(&mut vec![iotas.0; iotas.1]);

    Ok(state)
}

pub fn swap<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.1);
    state.stack.push(iotas.0);

    Ok(state)
}

pub fn rotate<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let iotas = (
        state.stack.get_iota(0, arg_count)?.clone(),
        state.stack.get_iota(1, arg_count)?.clone(),
        state.stack.get_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    state.stack.push(iotas.1);
    state.stack.push(iotas.2);
    state.stack.push(iotas.0);

    Ok(state)
}

mod tests {

    use crate::pattern_registry::PatternRegistryExt;

    use super::*;

    #[test]
    fn rotate_test() {
        let mut state = State::default();
        state.stack = vec![Iota::Number(0.0), Iota::Number(1.0), Iota::Number(2.0)];

        let expected = vec![Iota::Number(1.0), Iota::Number(2.0), Iota::Number(0.0)];

        let result = rotate(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }
}
