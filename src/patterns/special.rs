use crate::{
    interpreter::{
        interpret_action,
        mishap::Mishap,
        push_iota, push_pattern,
        state::{Either, StackExt, State},
    },
    iota::{EntityIota, Iota, PatternIota, PatternIotaExt, Signature},
    parser::ActionValue,
};

pub fn escape(state: &mut State) -> Result<&mut State, Mishap> {
    state.consider_next = true;
    Ok(state)
}

pub fn introspect(state: &mut State) -> Result<&mut State, Mishap> {
    let new_buffer = match &state.buffer {
        Some(buffer) => {
            let mut new_buffer = buffer.clone();
            new_buffer.push((
                Iota::Pattern(PatternIota::from_name(
                    &state.pattern_registry,
                    "open_paren",
                    None,
                )),
                false,
            ));
            new_buffer
        }
        None => vec![],
    };

    state.buffer = Some(new_buffer);

    Ok(state)
}

pub fn retrospect(state: &mut State) -> Result<&mut State, Mishap> {
    let inner_buffer = state.buffer.as_ref().ok_or(Mishap::HastyRetrospection)?;

    let intro_pattern = Iota::Pattern(PatternIota::from_name(
        &state.pattern_registry,
        "open_paren",
        None,
    ));
    let retro_pattern = Iota::Pattern(PatternIota::from_name(
        &state.pattern_registry,
        "close_paren",
        None,
    ));

    let intro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0 == intro_pattern && x.1 == false {
            acc + 1
        } else {
            acc
        }
    }) + 1;

    let retro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0 == retro_pattern && x.1 == false {
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
        push_pattern("close_paren".to_string(), None, state, false)
    };
    Ok(state)
}

pub fn no_action(state: &mut State) -> Result<&mut State, Mishap> {
    Ok(state)
}

pub fn halt(state: &mut State) -> Result<&mut State, Mishap> {
    state.halt = true;
    Ok(state)
}
