pub mod mishap;
mod ops;
pub mod state;

use std::collections::HashMap;

use crate::{
    interpreter::{
        ops::{embed, push, store, EmbedType},
        state::StackExt,
    },
    iota::{Iota, PatternIota, Signature, SignatureExt},
    parser::{ActionValue, AstNode},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{mishap::Mishap, state::State};

pub fn interpret(node: AstNode) -> Result<State, String> {
    let mut state = State {
        stack: vec![],
        ravenmind: None,
        buffer: None,
        heap: HashMap::new(),
        consider_next: false,
        halt: false,
    };
    let pattern_registry = PatternRegistry::construct();

    (interpret_node(node, &mut state, &pattern_registry)).map(|state| state.clone())
}

fn interpret_node<'a>(
    node: AstNode,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, String> {
    println!("a: {:?}, {:?}", state.stack, state.buffer);

    match node {
        AstNode::File(nodes) => {
            for node in nodes {
                interpret_node(node, state, pattern_registry)?;
                if state.halt {
                    break;
                }
            }
            Ok(state)
        }

        AstNode::Action { name, value } => interpret_action(name, value, state, pattern_registry)
            .map_err(|err| format!("{:?}", err)),
        AstNode::Hex(nodes) => {
            interpret_action("open_paren".to_string(), None, state, pattern_registry)
                .map_err(|err| format!("{:?}", err))?;
            for node in nodes {
                interpret_node(node, state, pattern_registry)?;
            }
            interpret_action("close_paren".to_string(), None, state, pattern_registry)
                .map_err(|err| format!("{:?}", err))?;

            Ok(state)
        }
        AstNode::Op { name, arg } => {
            match name {
                crate::parser::OpName::Store => store(&arg, state, false),
                crate::parser::OpName::Copy => store(&arg, state, true),
                crate::parser::OpName::Push => push(&arg, state),
                crate::parser::OpName::Embed => {
                    embed(&arg, state, pattern_registry, EmbedType::Normal)
                }
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
        } => {
            interpret_node(*condition, state, pattern_registry)?;

            let condition = state
                .stack
                .get_bool(0, 1)
                .map_err(|err| format!("{:?}", err))?;

            state.stack.remove_args(&1);

            if condition {
                interpret_node(*succeed, state, pattern_registry)?;
            } else if let Some(node) = fail {
                interpret_node(*node, state, pattern_registry)?;
            }

            Ok(state)
        }
    }
}

pub fn interpret_action<'a>(
    name: String,
    value: Option<ActionValue>,
    mut state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let pattern = pattern_registry.find(&name).ok_or(Mishap::InvalidPattern)?;

    let is_escape =
        Signature::from_sig(&pattern.signature) == Signature::from_name(pattern_registry, "escape");

    let is_retro = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "close_paren");

    let get_value_iota = || match &value {
        Some(ActionValue::Iota(iota)) => Some(iota),
        _ => None,
    };

    if state.consider_next {
        push_pattern(
            name,
            get_value_iota().cloned(),
            state,
            pattern_registry,
            true,
        );
        state.consider_next = false;
        return Ok(state)
    }

    if state.buffer.is_some() && !(is_escape || is_retro) {
        push_pattern(
            name,
            get_value_iota().cloned(),
            state,
            pattern_registry,
            false,
        );
        return Ok(state)
    }

    pattern.operate(state, pattern_registry, &value)?;

    if value.is_some() {
        match value.unwrap() {
            ActionValue::Iota(iota) => {
                push_iota(iota, state, is_escape);
            }
            ActionValue::Bookkeeper(_) => todo!(),
        };
    }
    
    return Ok(state)
}

pub fn push_pattern(
    pattern: String,
    value: Option<Iota>,
    state: &mut State,
    pattern_registry: &PatternRegistry,
    considered: bool,
) {
    push_iota(
        Iota::Pattern(PatternIota::from_name(pattern_registry, &pattern, value)),
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
