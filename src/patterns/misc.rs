use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::{EntityIota, Iota},
};

pub fn introspect(state: State) -> Result<State, Mishap> {
    let new_buffer = match state.buffer {
        Some(buffer) => {
            let mut new_buffer = buffer.clone();
            new_buffer.push((Iota::Pattern("Introspection".to_string()), false));
            new_buffer
        }
        None => vec![],
    };

    Ok(State {
        buffer: Some(new_buffer),
        ..state
    })
}
