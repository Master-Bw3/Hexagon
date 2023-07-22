use std::rc::Rc;

use im::vector;

use crate::{
    interpreter::{
        continuation::{iota_list_to_ast_node_list, FrameEndEval, FrameEvaluate, FrameForEach},
        mishap::Mishap,
        state::{Either3, StackExt, State},
    },
    iota::hex_casting::{
        continuation::ContinuationIota,
        list::ListIota,
        pattern::{PatternIota, SignatureExt},
    },
    parser::AstNode,
    pattern_registry::PatternRegistry,
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
            state.continuation.push_back(Rc::new(FrameEndEval {}));
            state.continuation.push_back(Rc::new(FrameEvaluate {
                nodes_queue: iota_list_to_ast_node_list(list),
            }));
        }
        Either3::M(pattern) => {
            state.continuation.push_back(Rc::new(FrameEvaluate {
                nodes_queue: vector![AstNode::Action {
                    line: (1, 0),
                    name: pattern.signature.as_str(),
                    value: *pattern.value.clone(),
                }],
            }));
        }
        Either3::R(continuation) => state.continuation = continuation.value.clone(),
    };

    Ok(state)
}

pub fn eval_cc<'a>(
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let continuation_iota = ContinuationIota {
        value: state.continuation.clone(),
    };
    eval(state, pattern_registry)?;
    state.stack.push_back(Rc::new(continuation_iota));

    Ok(state)
}

pub fn for_each<'a>(state: &'a mut State, _: &PatternRegistry) -> Result<&'a mut State, Mishap> {
    let arg_count = 2;
    let pattern_list = state.stack.get_iota::<ListIota>(0, 2)?;
    let iota_list = state.stack.get_iota::<ListIota>(1, 2)?;
    state.stack.remove_args(&arg_count);

    state.continuation.push_back(Rc::new(FrameForEach {
        data: (*iota_list).clone(),
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
