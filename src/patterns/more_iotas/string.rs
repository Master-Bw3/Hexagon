use std::rc::Rc;

use crate::iota::hex_casting::list::ListIota;
use crate::iota::more_iotas::string::StringVecExt;
use im::Vector;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{
        hex_casting::{
            null::NullIota,
            number::{NumberIota, NumberIotaExt},
            vector::VectorIota,
        },
        more_iotas::string::StringIota,
        Iota,
    },
    pattern_registry::PatternRegistry,
};

pub fn concat<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let iotas = (
        state.stack.get_iota::<StringIota>(0, arg_count)?,
        state.stack.get_iota::<StringIota>(1, arg_count)?,
    );
    state.stack.remove_args(&arg_count);

    let mut concatted = (*iotas.0).clone();
    concatted.push_str((iotas.1).as_ref());

    state.stack.push_back(Rc::new(concatted));

    Ok(state)
}

pub fn split<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;

    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    let delimiter = state.stack.get_iota::<StringIota>(1, arg_count)?;

    state.stack.remove_args(&arg_count);

    let strings = (*string)
        .clone()
        .split(delimiter.as_str())
        .map(|str| -> Rc<dyn Iota> { Rc::new(str.to_owned()) })
        .collect::<Vector<_>>();

    state.stack.push_back(Rc::new(strings));

    Ok(state)
}

pub fn parse<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let parse_result: Rc<dyn Iota> = match string.parse::<NumberIota>() {
        Ok(num) => Rc::new(num),
        Err(_) => Rc::new(NullIota),
    };

    state.stack.push_back(parse_result);

    Ok(state)
}

pub fn find<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    let query = state.stack.get_iota::<StringIota>(1, arg_count)?;
    state.stack.remove_args(&arg_count);

    let find_count = string.matches(query.as_str()).count() as NumberIota;

    state.stack.push_back(Rc::new(find_count));

    Ok(state)
}

pub fn sub<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    let start = state
        .stack
        .get_iota::<NumberIota>(1, arg_count)?
        .positive_int_under_inclusive(1, string.len())? as usize;
    let end = state
        .stack
        .get_iota::<NumberIota>(2, arg_count)?
        .positive_int_under_inclusive(2, string.len())? as usize;

    state.stack.remove_args(&arg_count);

    let find_count = string[start..end].to_owned();

    state.stack.push_back(Rc::new(find_count));

    Ok(state)
}

pub fn len<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let string = state.stack.get_iota::<StringIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    let len = string.len() as NumberIota;

    state.stack.push_back(Rc::new(len));

    Ok(state)
}

pub fn write<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    state.stack.get_iota::<VectorIota>(0, arg_count)?;
    //check for list of strings or a single string
    if let Ok(list) = state.stack.get_iota::<ListIota>(1, arg_count) {
        list.string_vec(1)?;
    } else {
        state.stack.get_iota::<StringIota>(1, arg_count)?;
    };

    state.stack.remove_args(&arg_count);

    Ok(state)
}
