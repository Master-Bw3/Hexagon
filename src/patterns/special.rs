use crate::{
    interpreter::{
        interpret_action,
        mishap::Mishap,
        push_iota, push_pattern,
        state::{Either, StackExt, State},
    },
    iota::{EntityIota, Iota, PatternIota, PatternIotaExt},
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

pub fn eval(state: &mut State) -> Result<&mut State, Mishap> {
    let arg_count = 1;
    let arg = state.stack.get_list_or_pattern(0, arg_count)?;
    state.stack.remove_args(1);
    match arg {
        Either::L(eval_list) => {
            
            //evaluate list
            for iota in eval_list {
                match iota {
                    Iota::Pattern(pattern) => {
                        interpret_action(
                            pattern.signature.as_str(),
                            pattern.value.map(|iota| ActionValue::Iota(iota)),
                            state,
                        )?;
                    }

                    iota => {
                        if state.consider_next || state.buffer.is_some() {
                            push_iota(iota, state, state.consider_next)
                        } else {
                            Err(Mishap::ExpectedPattern(iota))?
                        }
                    }
                }
            }

            state.buffer = None;
        }
        Either::R(pattern) => {
            //evaluate pattern
            interpret_action(
                pattern.signature.as_str(),
                pattern.value.map(|iota| ActionValue::Iota(iota)),
                state,
            )?;
        }
    };

    Ok(state)
}
