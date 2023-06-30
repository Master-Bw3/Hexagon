pub mod mishap;
mod ops;
pub mod state;

use std::collections::HashMap;

use crate::{
    interpreter::ops::{push, store},
    iota::{Iota, PatternIota, PatternIotaExt, Signature},
    parser::{ActionValue, AstNode},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{state::State, mishap::Mishap};

pub fn interpret(node: AstNode) -> Result<State, String> {
    let mut state = State {
        stack: vec![],
        ravenmind: None,
        buffer: None,
        heap: HashMap::new(),
        consider_next: false,
        pattern_registry: PatternRegistry::construct(),
    };

    (interpret_node(node, &mut state)).cloned()
}

fn interpret_node<'a>(node: AstNode, mut state: &'a mut State) -> Result<&'a mut State, String> {
    println!("a: {:?}, {:?}", state.stack, state.buffer);

    match node {
        AstNode::Action { name, value } => interpret_action(name, value, state).map_err(|err| format!("{:?}", err)),
        AstNode::Hex(nodes) => {
            for node in nodes {
                interpret_node(node, state)?;
            }
            Ok(state)
        }
        AstNode::Op { name, arg } => {
            match name {
                crate::parser::OpName::Store => store(&arg, state, false),
                crate::parser::OpName::Copy => store(&arg, state, true),
                crate::parser::OpName::Push => push(&arg, state),
                crate::parser::OpName::Embed => todo!(),
                crate::parser::OpName::SmartEmbed => todo!(),
                crate::parser::OpName::ConsiderEmbed => todo!(),
                crate::parser::OpName::IntroEmbed => todo!(),
            }?;

            Ok(state)
        }
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
        } => todo!(),
    }
}

pub fn interpret_action<'a>(
    name: String,
    value: Option<ActionValue>,
    mut state: &'a mut State,
) -> Result<&'a mut State, Mishap> {
    let is_escape = Signature::from_name(&state.pattern_registry, &name)
        == Signature::from_name(&state.pattern_registry, "escape");

    let is_retro = Signature::from_name(&state.pattern_registry, &name)
        == Signature::from_name(&state.pattern_registry, "close_paren");

    let is_eval = Signature::from_name(&state.pattern_registry, &name)
        == Signature::from_name(&state.pattern_registry, "eval");

    let get_value_iota = || match &value {
        Some(ActionValue::Iota(iota)) => Some(iota),
        _ => None,
    };

    {
        if state.consider_next {
            push_pattern(name, get_value_iota().cloned(), state, true);
            state.consider_next = false;
            Ok(state)
        } else if state.buffer.is_some() && !(is_escape || is_retro) {
            push_pattern(name, get_value_iota().cloned(), state, false);
            Ok(state)
        } else if is_eval {
            todo!()
        } else {
            match value {
                Some(val) => match val {
                    ActionValue::Iota(iota) => {
                        push_iota(iota, state, is_escape);
                        Ok(state)
                    }
                    ActionValue::Bookkeeper(_) => todo!(),
                },
                None => {
                    let pattern = state
                        .pattern_registry
                        .find(name)
                        .ok_or(Mishap::InvalidPattern)?
                        .clone();

                    pattern
                        .operate(state, value)
                }
            }
        }
    }
}

pub fn push_pattern(pattern: String, value: Option<Iota>, state: &mut State, considered: bool) {
    push_iota(
        Iota::Pattern(PatternIota::from_name(
            &state.pattern_registry,
            &pattern,
            value,
        )),
        state,
        considered,
    )
}

pub fn push_iota(iota: Iota, state: &mut State, considered: bool) {
    match state.buffer {
        Some(ref mut buffer) => buffer.push((iota, considered)),
        None => state.stack.push(iota),
    }
}
