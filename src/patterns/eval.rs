use crate::{
    interpreter::{
        self,
        mishap::Mishap,
        state::{Either, StackExt, State},
    },
    iota::{Iota, PatternIota, Signature, SignatureExt},
    parser::ActionValue,
    pattern_registry::PatternRegistry,
};

pub fn eval<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let arg = state.stack.get_list_or_pattern(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    match arg {
        Either::L(list) => {
            eval_list(state, pattern_registry, &list)?;
        }
        Either::R(pattern) => {
            eval_pattern(state, pattern_registry, &pattern)?;
        }
    };

    Ok(state)
}

type Halted = bool;

fn eval_list(
    state: &mut State,
    pattern_registry: &PatternRegistry,
    list: &Vec<Iota>,
) -> Result<Halted, Mishap> {
    
    let mut halted = false;
    for iota in list {
        match iota {
            Iota::Pattern(pattern) => {
                if pattern.signature == Signature::from_name(pattern_registry, "halt") {
                    halted = true;
                    break;
                }
                eval_pattern(state, pattern_registry, pattern)?;
            }

            iota => {
                if state.consider_next || state.buffer.is_some() {
                    interpreter::push_iota(iota.clone(), state, state.consider_next)
                } else {
                    Err(Mishap::ExpectedPattern(iota.clone()))?
                }
            }
        }
    }

    state.buffer = None;
    Ok(halted)
}

fn eval_pattern(
    state: &mut State,
    pattern_registry: &PatternRegistry,
    pattern: &PatternIota,
) -> Result<(), Mishap> {
    interpreter::interpret_action(
        pattern.signature.as_str(),
        pattern.value.clone().map(|iota| ActionValue::Iota(iota)),
        state,
        pattern_registry,
    )?;
    Ok(())
}

pub fn for_each<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    
    let arg_count = 2;
    let pattern_list = state.stack.get_list(0, 2)?;
    let iota_list = state.stack.get_list(1, 2)?;
    state.stack.remove_args(&arg_count);

    for iota in iota_list {
        let mut temp_state = state.clone();
        temp_state.stack.push(iota);

        let halted = eval_list(&mut temp_state, pattern_registry, &pattern_list)?;

        if halted {
            break;
        }

        //update state
        temp_state.stack = state.stack.clone();
        std::mem::swap(state, &mut temp_state);
    }

    Ok(state)
}
