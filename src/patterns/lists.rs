use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{Iota, NullIota::Null},
    pattern_registry::PatternRegistry,
};

pub fn append<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        state.stack.get_list(0, arg_count)?,
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    (iotas.0).push(iotas.1);
    state.stack.push(Iota::List(iotas.0));

    Ok(state)
}

pub fn concat<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        state.stack.get_list(0, arg_count)?,
        state.stack.get_list(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    (iotas.0).append(&mut iotas.1);
    state.stack.push(Iota::List(iotas.0));

    Ok(state)
}

pub fn index<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_list(0, arg_count)?,
        state.stack.get_number(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operaton_result = (iotas.0)
        .get((iotas.1).round() as usize)
        .unwrap_or(&Iota::Null(Null));

    state.stack.push(operaton_result.clone());

    Ok(state)
}

pub fn list_size<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_list(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let operaton_result = iota.len() as f32;

    state.stack.push(Iota::Number(operaton_result));

    Ok(state)
}

pub fn singleton<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_iota(0, arg_count)?.clone();
    state.stack.remove_args(&arg_count);

    state.stack.push(Iota::List(vec![iota]));

    Ok(state)
}

pub fn reverse_list<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let mut iota = state.stack.get_list(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    iota.reverse();
    state.stack.push(Iota::List(iota));

    Ok(state)
}

pub fn last_n_list<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let list_arg_count = (state.stack.get_number(0, 1)?.round()) as usize;

    let mut iotas = vec![];

    for index in 0..(list_arg_count) {
        iotas.push(state.stack.get_iota(index, list_arg_count + 1)?.clone());
    }
    state.stack.remove_args(&(list_arg_count + 1));

    state.stack.push(Iota::List(iotas));

    Ok(state)
}

pub fn splat<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let mut iota = state.stack.get_list(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    state.stack.append(&mut iota);

    Ok(state)
}

pub fn index_of<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_list(0, arg_count)?,
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let operation_result = (iotas.0)
        .iter()
        .enumerate()
        .filter(|(_, x)| x.check_equality(&iotas.1))
        .collect::<Vec<(usize, &Iota)>>()
        .get(0)
        .map(|x| x.0 as f32)
        .unwrap_or(-1.0);
    state.stack.push(Iota::Number(operation_result));

    Ok(state)
}

pub fn list_remove<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        state.stack.get_list(0, arg_count)?,
        state.stack.get_integer(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let remove_index = i32::min(iotas.1, ((iotas.0).len() - 1) as i32);
    (iotas.0).remove(remove_index as usize);

    state.stack.push(Iota::List(iotas.0));

    Ok(state)
}

pub fn slice<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let list = state.stack.get_list(0, arg_count)?;

    let iotas = (
        (state.stack).get_positive_integer_under_inclusive(1, list.len(), arg_count)? as usize,
        (state.stack).get_positive_integer_under_inclusive(2, list.len(), arg_count)? as usize,
    );
    state.stack.remove_args(&arg_count);

    state
        .stack
        .push(Iota::List(list[iotas.0..iotas.1].to_vec()));

    Ok(state)
}

pub fn modify_in_place<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let mut list = state.stack.get_list(0, arg_count)?;

    let iotas = (
        (state.stack).get_positive_integer_under_inclusive(1, list.len(), arg_count)? as usize,
        (state.stack).get_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    list.remove(iotas.0);
    list.insert(iotas.0, iotas.1);

    state.stack.push(Iota::List(list));

    Ok(state)
}

pub fn construct<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        state.stack.get_list(0, arg_count)?,
        state.stack.get_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let operation_result = {
        let mut new_list = vec![iotas.1];
        new_list.append(&mut iotas.0);
        new_list
    };

    state.stack.push(Iota::List(operation_result));

    Ok(state)
}

pub fn deconstruct<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let mut iota = state.stack.get_list(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    iota.remove(0);
    state.stack.push(Iota::List(iota));

    Ok(state)
}

#[cfg(test)]
mod tests {

    use crate::pattern_registry::PatternRegistryExt;

    use super::*;

    #[test]
    fn last_n_list_test() {
        let mut state = State::default();
        state.stack = vec![
            Iota::Number(1.0),
            Iota::Number(1.0),
            Iota::Number(2.0),
            Iota::Number(3.0),
        ];

        let expected = vec![Iota::List(vec![
            Iota::Number(1.0),
            Iota::Number(1.0),
            Iota::Number(2.0),
        ])];

        let result = last_n_list(&mut state, &PatternRegistry::construct()).unwrap();
        assert_eq!(result.stack, expected)
    }
}
