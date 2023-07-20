use std::rc::Rc;

use im::{Vector, vector};

use crate::{
    interpreter::{
        self,
        continuation::{iota_list_to_ast_node_list, FrameEndEval, FrameEvaluate, FrameForEach},
        mishap::Mishap,
        state::{Either3, StackExt, State},
    },
    parser::{AstNode},
    pattern_registry::PatternRegistry, iota::{Iota, hex_casting::{pattern::{Signature, PatternIota, SignatureExt}, list::ListIota, continuation::ContinuationIota}},
};


pub fn eval<'a>(
    state: &'a mut State,
    _pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let arg_count = 1;
    let arg = state
        .stack
        .get_iota_a_b_or_c::<ListIota, PatternIota, ContinuationIota>(0, arg_count)?;
    state.stack.remove_args(&arg_count);

    match arg {
        Either3::L(list) => {
            state.continuation.push(Rc::new(FrameEndEval {}));
            state.continuation.push(Rc::new(FrameEvaluate {
                nodes: iota_list_to_ast_node_list(list),
            }));
        }
        Either3::M(pattern) => {
            state.continuation.push(Rc::new(FrameEvaluate {
                nodes: vec![AstNode::Action {
                    line: (1, 0),
                    name: pattern.signature.as_str(),
                    value: *pattern.value,
                }],
            }));
        }
        Either3::R(continuation) => state.continuation = *continuation,
    };

    Ok(state)
}

pub fn eval_cc<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let continuation_iota = state.continuation.clone();
    eval(state, pattern_registry)?;
    state.stack.push_back(Rc::new(continuation_iota));

    Ok(state)
}

type Halted = bool;

fn eval_list(
    state: &mut State,
    pattern_registry: &PatternRegistry,
    list: &[Rc<dyn Iota>],
) -> Result<Halted, Box<Mishap>> {
    let mut halted = false;
    for (index, iota) in list.iter().enumerate() {
        
        match iota.downcast_rc::<PatternIota>() {
            Ok(pattern) => {
                if pattern.signature
                    == Signature::from_name(pattern_registry, "halt", &None).unwrap()
                {
                    halted = true;
                    break;
                }
                eval_pattern(state, pattern_registry, pattern.as_ref())
                    .map_err(|err| Mishap::EvalError(list.to_owned(), index, Rc::new(err)))?;
            }

            Err(_) => {
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

pub fn for_each<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let pattern_list = state.stack.get_iota::<ListIota>(0, 2)?;
    let mut iota_list = state.stack.get_iota::<ListIota>(1, 2)?;
    state.stack.remove_args(&arg_count);

    

    state.continuation.push(Rc::new(FrameForEach {
        data: iota_list.into_iter().rev().collect(),
        code: iota_list_to_ast_node_list(pattern_list),
        base_stack: None,
        acc: vector![],
    }));

    Ok(state)
}

pub fn halt<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    let mut done = false;
    while !(done || state.continuation.is_empty()) {
        done = state.continuation.last().unwrap().clone().break_out(state);
    }
    Ok(state)
}
