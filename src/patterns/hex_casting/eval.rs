use crate::{
    interpreter::{
        self,
        mishap::Mishap,
        state::{Either, Either3, StackExt, State},
    },
    iota::{Iota, PatternIota, Signature, SignatureExt},
    parser::{AstNode, OpName, OpValue, Instruction},
    pattern_registry::PatternRegistry,
};
use owo_colors::OwoColorize;

pub fn eval<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let arg = state
        .stack
        .get_list_or_pattern_or_continuation(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    match arg {
        Either3::L(list) => {
            state.continuation.push(AstNode::Instruction(Instruction::MetaEvalEnd));
            state.continuation.append(
                &mut list
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(index, iota)| match iota {
                        Iota::Pattern(pattern) => AstNode::Action {
                            line: (index + 1, 0),
                            name: pattern.signature.as_str(),
                            value: *pattern.value.clone(),
                        },
                        _ => AstNode::Op {
                            line: (index, 0),
                            name: OpName::Embed,
                            arg: Some(OpValue::Iota(iota.clone())),
                        },
                    })
                    .collect(),
            );
        }
        Either3::M(pattern) => {
            state.continuation.push(
                AstNode::Action {
                    line: (1, 0),
                    name: pattern.signature.as_str(),
                    value: *pattern.value,
                },
            );
        }
        Either3::R(continuation) => state.continuation = continuation,
    };

    Ok(state)
}

pub fn eval_cc<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let continuation_iota = Iota::Continuation(state.continuation.clone());
    eval(state, pattern_registry)?;
    state.stack.push(continuation_iota);

    Ok(state)
}

type Halted = bool;

fn eval_list(
    state: &mut State,
    pattern_registry: &PatternRegistry,
    list: &Vec<Iota>,
) -> Result<Halted, Mishap> {
    let mut halted = false;
    for (index, iota) in list.iter().enumerate() {
        match iota {
            Iota::Pattern(pattern) => {
                if pattern.signature
                    == Signature::from_name(pattern_registry, "halt", &None).unwrap()
                {
                    halted = true;
                    break;
                }
                eval_pattern(state, pattern_registry, pattern)
                    .map_err(|err| Mishap::EvalMishap(list.clone(), index, Box::new(err)))?;
            }

            iota => {
                if state.consider_next || state.buffer.is_some() {
                    interpreter::push_iota(iota.clone(), state, state.consider_next);
                    state.consider_next = false;
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
        *pattern.value.clone(),
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

    let mut result = vec![];

    for iota in iota_list {
        let mut temp_state = state.clone();
        temp_state.stack.push(iota);

        let halted = eval_list(&mut temp_state, pattern_registry, &pattern_list)?;

        if halted {
            break;
        }

        result.append(&mut temp_state.stack);
        //update state
        temp_state.stack = state.stack.clone();
        *state = temp_state;
    }
    state.stack.push(Iota::List(result));

    Ok(state)
}
