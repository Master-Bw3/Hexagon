use std::rc::Rc;

use im::{vector, Vector};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::hex_casting::{
        list::ListIota,
        null::NullIota,
        number::{NumberIota, NumberIotaExt},
    },
    pattern_registry::PatternRegistry,
};

pub fn append<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    (iotas.0).push_back(iotas.1);
    state.stack.push_back(Rc::new(iotas.0));

    Ok(state)
}

pub fn concat<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone(),
        (*state.stack.get_iota::<ListIota>(1, arg_count)?).clone(),
    );
    state.stack.remove_args(&arg_count);

    (iotas.0).append(iotas.1);
    state.stack.push_back(Rc::new(iotas.0));

    Ok(state)
}

pub fn index<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone(),
        state.stack.get_iota::<NumberIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let operaton_result = (iotas.0)
        .get((iotas.1).round() as usize)
        .cloned()
        .unwrap_or(Rc::new(NullIota));

    state.stack.push_back(operaton_result.clone());

    Ok(state)
}

pub fn list_size<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    let operaton_result = iota.len() as f32;

    state.stack.push_back(Rc::new(operaton_result));

    Ok(state)
}

pub fn singleton<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = state.stack.get_any_iota(0, arg_count)?.clone();
    state.stack.remove_args(&arg_count);

    state.stack.push_back(Rc::new(vector![iota]));

    Ok(state)
}

pub fn reverse_list<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    let reversed_list = iota.into_iter().rev().collect::<Vector<_>>();

    state.stack.push_back(Rc::new(reversed_list));

    Ok(state)
}

pub fn last_n_list<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let list_arg_count = (state.stack.get_iota::<NumberIota>(0, 1)?.round()) as usize;

    let mut iotas = vector![];

    for index in 0..(list_arg_count) {
        iotas.push_back(state.stack.get_any_iota(index, list_arg_count + 1)?.clone());
    }
    state.stack.remove_args(&(list_arg_count + 1));

    state.stack.push_back(Rc::new(iotas));

    Ok(state)
}

pub fn splat<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let iota = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    state.stack.append(iota);

    Ok(state)
}

pub fn index_of<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let operation_result = (iotas.0)
        .iter()
        .enumerate()
        .filter(|(_, x)| x.tolerates_other(iotas.1.as_ref()))
        .collect::<Vector<(usize, _)>>()
        .get(0)
        .map(|x| x.0 as f32)
        .unwrap_or(-1.0);
    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}

pub fn list_remove<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let mut iotas = (
        (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone(),
        state.stack.get_iota::<NumberIota>(1, arg_count)?.int(1)?,
    );
    state.stack.remove_args(&arg_count);

    let remove_index = i32::min(iotas.1, ((iotas.0).len() - 1) as i32);
    (iotas.0).remove(remove_index as usize);

    state.stack.push_back(Rc::new(iotas.0));

    Ok(state)
}

pub fn slice<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let mut list = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();

    let iotas = (
        (state.stack)
            .get_iota::<NumberIota>(1, arg_count)?
            .int_under_inclusive(1, list.len())? as usize,
        (state.stack)
            .get_iota::<NumberIota>(2, arg_count)?
            .int_under_inclusive(2, list.len())? as usize,
    );
    state.stack.remove_args(&arg_count);

    state.stack.push_back(Rc::new(list.slice(iotas.0..iotas.1)));

    Ok(state)
}

pub fn modify_in_place<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let mut list = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();

    let iotas = (
        (state.stack)
            .get_iota::<NumberIota>(1, arg_count)?
            .int_under_inclusive(1, list.len())? as usize,
        (state.stack).get_any_iota(2, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    list.remove(iotas.0);
    list.insert(iotas.0, iotas.1);

    state.stack.push_back(Rc::new(list));

    Ok(state)
}

pub fn construct<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone(),
        state.stack.get_any_iota(1, arg_count)?.clone(),
    );
    state.stack.remove_args(&arg_count);

    let operation_result = {
        let mut new_list = vector![iotas.1];
        new_list.append(iotas.0);
        new_list
    };

    state.stack.push_back(Rc::new(operation_result));

    Ok(state)
}

pub fn deconstruct<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let mut iota = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    let taken = iota[0].clone();
    iota.remove(0);

    state.stack.push_back(Rc::new(iota));
    state.stack.push_back(taken);

    Ok(state)
}

#[cfg(test)]
mod tests {

    use crate::{pattern_registry::PatternRegistryExt, iota::Iota};

    use super::*;

    #[test]
    fn last_n_list_test() {
        let mut state = State::default();
        state.stack = vector![1.0, 1.0, 2.0, 3.0,]
            .into_iter()
            .map(|x: f32| -> Rc<dyn Iota> { Rc::new(x) })
            .collect();

        let expected: Vector<Rc<dyn Iota>> = vector![1.0, 1.0, 2.0,]
            .into_iter()
            .map(|x: f32| -> Rc<dyn Iota> { Rc::new(x) })
            .collect();

        let result = last_n_list(
            &mut state,
            &PatternRegistry::construct(&PatternRegistry::gen_default_great_sigs()),
        )
        .unwrap();
        assert!(result.stack.tolerates_other(&expected))
    }
}
