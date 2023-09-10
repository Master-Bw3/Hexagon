use std::rc::Rc;

use im::{vector, Vector};

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{
        hex_casting::{list::ListIota, number::{NumberIota, NumberIotaExt}},
        Iota,
    },
    pattern_registry::PatternRegistry,
};

pub fn running_sum<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let list = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    let mut num_list = vector![];

    for iota in list {
        let num = iota
            .clone()
            .downcast_rc::<NumberIota>()
            .map_err(|_| Mishap::IncorrectIota(0, "List of numbers".to_string(), iota.clone()))?;

        num_list.push_back(*num)
    }

    for i in 1..num_list.len() {
        num_list[i] = num_list[i] + num_list[i - 1];
    }

    let result: Vector<_> = num_list
        .into_iter()
        .map(|num| -> Rc<dyn Iota> { Rc::new(num) })
        .collect();

    state.stack.push_back(Rc::new(result));

    Ok(state)
}

pub fn running_product<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let list = (*state.stack.get_iota::<ListIota>(0, arg_count)?).clone();
    state.stack.remove_args(&arg_count);

    let mut num_list = vector![];

    for iota in list {
        let num = iota
            .clone()
            .downcast_rc::<NumberIota>()
            .map_err(|_| Mishap::IncorrectIota(0, "List of numbers".to_string(), iota.clone()))?;

        num_list.push_back(*num)
    }

    for i in 1..num_list.len() {
        num_list[i] = num_list[i] * num_list[i - 1];
    }

    let result: Vector<_> = num_list
        .into_iter()
        .map(|num| -> Rc<dyn Iota> { Rc::new(num) })
        .collect();

    state.stack.push_back(Rc::new(result));

    Ok(state)
}

pub fn factorial<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let num = state.stack.get_iota::<NumberIota>(0, arg_count)?.positive_int(0)? as usize;
    state.stack.remove_args(&arg_count);

    let result = (1..=num).product::<usize>() as f64;

    state.stack.push_back(Rc::new(result));

    Ok(state)
}
