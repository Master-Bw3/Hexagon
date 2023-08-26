use std::{rc::Rc, cell::RefCell};

use im::vector;

use crate::{
    interpreter::{
        continuation::{
            iota_list_to_ast_node_list, ContinuationFrame, FrameEndEval, FrameEvaluate,
            FrameForEach, ContinuationFrameTrait,
        },
        mishap::Mishap,
        state::{Either3, StackExt, State},
    },
    iota::hex_casting::{
        continuation::ContinuationIota,
        list::ListIota,
        pattern::{PatternIota, SignatureExt},
    },
    parser::{AstNode, Location},
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
            state
                .continuation
                .push_back(ContinuationFrame::EndEval(FrameEndEval {}));
            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: iota_list_to_ast_node_list(list),
                }));
        }
        Either3::M(pattern) => {
            state.continuation.push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                nodes_queue: vector![AstNode::Action {
                    location: Location::List(0),
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

    state.continuation.push_back(ContinuationFrame::ForEach(FrameForEach {
        data: (*iota_list).clone(),
        code: iota_list_to_ast_node_list(pattern_list),
        base_stack: None,
        acc: Rc::new(RefCell::new(vector![])),
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
