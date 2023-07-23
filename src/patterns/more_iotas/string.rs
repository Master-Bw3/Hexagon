use std::rc::Rc;

use im::Vector;

use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{more_iotas::string::StringIota, Iota},
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
        .map(|str| -> Rc<dyn Iota> { Rc::new(str.to_string()) })
        .collect::<Vector<_>>();

    state.stack.push_back(Rc::new(strings));

    Ok(state)
}
