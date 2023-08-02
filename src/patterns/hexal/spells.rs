use crate::{
    interpreter::{
        mishap::Mishap,
        state::{StackExt, State},
    },
    iota::hex_casting::{list::ListIota, vector::VectorIota},
    pattern_registry::PatternRegistry,
};

pub fn particles<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let arg = state
        .stack
        .get_iota_a_or_b::<VectorIota, ListIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    match arg {
        crate::interpreter::state::Either::L(_) => (),
        crate::interpreter::state::Either::R(list) => {
            for iota in (*list).clone() {
                iota.clone().downcast_rc::<VectorIota>().map_err(|_| {
                    Mishap::IncorrectIota(0, "List of vectors".to_string(), iota.clone())
                })?;
            }
        }
    }

    Ok(state)
}
