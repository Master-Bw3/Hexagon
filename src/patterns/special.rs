use rand::{seq::SliceRandom, thread_rng};

use crate::{
    interpreter::{
        mishap::Mishap,
        push_pattern,
        state::{StackExt, State},
    },
    iota::{Iota, PatternIota},
    parser::ActionValue,
    pattern_registry::PatternRegistry,
};

pub fn escape<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    state.consider_next = true;
    Ok(state)
}

pub fn introspect<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let new_buffer = match &state.buffer {
        Some(buffer) => {
            let mut new_buffer = buffer.clone();
            new_buffer.push((
                Iota::Pattern(PatternIota::from_name(pattern_registry, "open_paren", None)),
                false,
            ));
            new_buffer
        }
        None => vec![],
    };

    state.buffer = Some(new_buffer);

    Ok(state)
}

pub fn retrospect<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let inner_buffer = state.buffer.as_ref().ok_or(Mishap::HastyRetrospection)?;

    let intro_pattern = Iota::Pattern(PatternIota::from_name(pattern_registry, "open_paren", None));
    let retro_pattern = Iota::Pattern(PatternIota::from_name(
        pattern_registry,
        "close_paren",
        None,
    ));

    let intro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0 == intro_pattern && !x.1 {
            acc + 1
        } else {
            acc
        }
    }) + 1;

    let retro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0 == retro_pattern && !x.1 {
            acc + 1
        } else {
            acc
        }
    }) + 1;

    if intro_count == retro_count {
        state.stack.push(Iota::List(
            inner_buffer
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<Iota>>(),
        ));
        state.buffer = None
    } else {
        push_pattern(
            "close_paren".to_string(),
            None,
            state,
            pattern_registry,
            false,
        )
    };
    Ok(state)
}

pub fn no_action<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    Ok(state)
}

pub fn halt<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    state.halt = true;
    Ok(state)
}

pub fn print<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let iota = state.stack.get_iota(0, 1)?;
    println!("{:?}", iota);
    Ok(state)
}

pub fn beep<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 3;
    state.stack.remove_args(&arg_count);

    let notes = ["beep", "boop"];
    let mut rng = thread_rng();

    let note = notes.choose(&mut rng).unwrap();
    println!("{note}");

    Ok(state)
}

pub fn mask<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
    value: &ActionValue,
) -> Result<&'a mut State, Mishap> {
    let code = match value {
        ActionValue::Bookkeeper(code) => code,
        _ => Err(Mishap::InvalidValue)?,
    };

    let apply_code = |(iota, char): (&Iota, char)| match char {
        '-' => Some(iota.clone()),
        'v' => None,
        _ => unreachable!(),
    };

    if state.stack.len() < code.len() {
        return Err(Mishap::NotEnoughIotas(code.len()));
    }

    let mut new_stack = state.stack[..state.stack.len() - code.len()].to_vec();
    let top_stack = state.stack[state.stack.len() - code.len()..].to_vec();
    let apply_result = &mut top_stack.iter().zip(code.chars()).filter_map(apply_code).collect::<Vec<_>>();

    new_stack.append(apply_result);

    state.stack = new_stack;

    Ok(state)
}
