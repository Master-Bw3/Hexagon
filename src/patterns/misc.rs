use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{EntityIota, Iota, PatternIota, PatternIotaExt},
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
                    "Introspection",
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
        "Introspection",
    ));
    let retro_pattern = Iota::Pattern(PatternIota::from_name(
        &state.pattern_registry,
        "Retrospection",
    ));

    let intro_count: i32 = inner_buffer.iter().fold(0, |acc, x| {
        if x.0 == retro_pattern && x.1 == false {
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

    
}
