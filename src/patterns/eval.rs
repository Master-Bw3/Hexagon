use crate::{
    interpreter::{
        self,
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{Iota, PatternIota, PatternIotaExt, Signature},
    parser::ActionValue,
};


pub fn eval(state: &mut State) -> Result<&mut State, Mishap> {
    let arg_count = 1;
    let arg = state.stack.get_list_or_pattern(0, arg_count)?;
    state.stack.remove_args(arg_count);

    match arg {
        Either::L(list) => {
            eval_list(state, list)?;
        }
        Either::R(pattern) => {
            eval_pattern(state, pattern)?;
        }
    };

    Ok(state)
}


fn eval_list(state: &mut State, list: Vec<Iota>) -> Result<(), Mishap> {
    for iota in list {
        match iota {
            Iota::Pattern(pattern) => {
                if pattern.signature == Signature::from_name(&state.pattern_registry, "halt") {
                    break;
                }
                eval_pattern(state, pattern)?;
            }

            iota => {
                if state.consider_next || state.buffer.is_some() {
                    interpreter::push_iota(iota, state, state.consider_next)
                } else {
                    Err(Mishap::ExpectedPattern(iota))?
                }
            }
        }
    }

    state.buffer = None;
    Ok(())
}


fn eval_pattern(state: &mut State, pattern: PatternIota) -> Result<(), Mishap> {
    interpreter::interpret_action(
        pattern.signature.as_str(),
        pattern.value.map(|iota| ActionValue::Iota(iota)),
        state,
    )?;
    Ok(())
}
