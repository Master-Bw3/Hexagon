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
            new_buffer.push((Iota::Pattern(PatternIota::from_name(&state.pattern_registry, "Introspection")), false));
            new_buffer
        }
        None => vec![],
    };

    state.buffer = Some(new_buffer);

    Ok(state)
}

pub fn retrospect(state: &mut State) -> Result<&mut State, Mishap> {
    let inner_buffer = state.buffer.as_ref().ok_or(Mishap::HastyRetrospection)?;
    let intro_count = inner_buffer
        .iter()
        .filter(|x| x.0 == Iota::Pattern(PatternIota::from_name(&state.pattern_registry, "Retrospection")));
    todo!()
}
