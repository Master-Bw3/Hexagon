pub mod mishap;
mod ops;
pub mod state;

use std::collections::HashMap;

use crate::{
    interpreter::ops::{push, store},
    iota::{Iota, PatternIota, PatternIotaExt},
    parser::{ActionValue, AstNode},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
    patterns::pattern::{self, Pattern},
};

use self::state::State;

pub fn interpret(node: AstNode) -> Result<State, String> {
    let mut state = State {
        stack: vec![],
        ravenmind: None,
        buffer: None,
        consider_next: false,
        pattern_registry: PatternRegistry::construct(),
    };
    let mut heap: HashMap<String, i32> = HashMap::new();

    (interpret_node(node, &mut state, &mut heap)).cloned()
}

fn interpret_node<'a>(
    node: AstNode,
    mut state: &'a mut State,
    heap: &mut HashMap<String, i32>,
) -> Result<&'a mut State, String> {
    println!("{:?}", state.stack);
    match node {
        AstNode::Action { name, value } => interpret_action(name, value, state, heap),
        AstNode::Hex(nodes) => {
            for node in nodes {
                state = interpret_node(node, state, heap)?;
            }
            Ok(state)
        }
        AstNode::Op { name, arg } => {
            match name {
                crate::parser::OpName::Store => store(&arg, state, heap, false),
                crate::parser::OpName::Copy => store(&arg, state, heap, true),
                crate::parser::OpName::Push => push(&arg, state, heap),
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

fn interpret_action<'a>(
    name: String,
    value: Option<ActionValue>,
    mut state: &'a mut State,
    heap: &mut HashMap<String, i32>,
) -> Result<&'a mut State, String> {
    let not_escape = PatternIota::from_name(&state.pattern_registry, &name)
        != PatternIota::from_name(&state.pattern_registry, "escape");

    let not_retro = PatternIota::from_name(&state.pattern_registry, &name)
        != PatternIota::from_name(&state.pattern_registry, "close_paren");

    {
        if state.consider_next {
            push_pattern(name, state, true);
            state.consider_next = false;
            Ok(state)
        } else if state.buffer.is_some() && not_escape {
            push_pattern(name, state, false);
            Ok(state)
        } else {
            match value {
                Some(val) => match val {
                    ActionValue::Iota(iota) => {
                        state.stack.push(iota);
                        Ok(state)
                    }
                    ActionValue::Bookkeeper(_) => todo!(),
                },
                None => {
                    let pattern = state
                        .pattern_registry
                        .find(name)
                        .ok_or("Invalid Action")?
                        .clone();

                    pattern
                        .operate(state, value)
                        .map_err(|err: mishap::Mishap| format!("{:?}", err))
                }
            }
        }
    }
}

pub fn push_pattern(pattern: String, state: &mut State, considered: bool) {
    push_iota(
        Iota::Pattern(PatternIota::from_name(&state.pattern_registry, &pattern)),
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
