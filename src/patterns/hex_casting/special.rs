use std::rc::Rc;

use im::{vector, Vector};
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    interpreter::{
        mishap::Mishap,
        push_pattern,
        state::{StackExt, State},
    },
    iota::hex_casting::pattern::PatternIota,
    parser::{ActionValue, Location},
    pattern_registry::PatternRegistry,
};

pub fn escape<'a>(
    state: &'a mut State,
    _: &PatternRegistry,
    value: Option<&ActionValue>,
) -> Result<&'a mut State, Mishap> {
    match value {
        Some(ActionValue::Iota(iota)) => state.stack.push_back(iota.clone()),
        Some(ActionValue::Bookkeeper(val)) => {
            Err(Mishap::InvalidValue{received: val.clone(), expected: "Iota".to_string()})?
        }
        None => {
            state.consider_next = true;
        }
    };

    Ok(state)
}

pub fn introspect<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let new_buffer = match &state.buffer {
        Some(buffer) => {
            let mut new_buffer = buffer.clone();
            new_buffer.push_back((
                Rc::new(
                    PatternIota::from_name(pattern_registry, "open_paren", None, Location::Unknown)
                        .unwrap(),
                ),
                false,
            ));
            new_buffer
        }
        None => vector![],
    };

    state.buffer = Some(new_buffer);
    Ok(state)
}

pub fn retrospect<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let inner_buffer = state.buffer.as_ref().ok_or(Mishap::HastyRetrospection)?;

    let intro_pattern =
        PatternIota::from_name(pattern_registry, "open_paren", None, Location::Unknown).unwrap();
    let retro_pattern =
        PatternIota::from_name(pattern_registry, "close_paren", None, Location::Unknown).unwrap();

    let intro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0.tolerates_other(&intro_pattern) && !x.1 {
            acc + 1
        } else {
            acc
        }
    }) + 1;

    let retro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0.tolerates_other(&retro_pattern) && !x.1 {
            acc + 1
        } else {
            acc
        }
    }) + 1;

    if intro_count == retro_count {
        state.stack.push_back(Rc::new(
            inner_buffer
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vector<_>>(),
        ));
        state.buffer = None
    } else {
        push_pattern(
            "close_paren".to_string(),
            None,
            state,
            pattern_registry,
            false,
            Location::Unknown,
        )
    };

    Ok(state)
}

pub fn no_action<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    Ok(state)
}

pub fn print<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let iota = state.stack.get_any_iota(0, 1)?;
    println!("{}", iota.display());
    Ok(state)
}

pub fn beep<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    if state.stack.len() < arg_count {
        Err(Mishap::NotEnoughIotas{arg_count, stack_height: state.stack.len()})?
    } else {
        state.stack.remove_args(&arg_count);
    }

    let notes = ["beep", "boop"];
    let mut rng = thread_rng();

    let note = notes.choose(&mut rng).unwrap();
    println!("{note}");

    Ok(state)
}
