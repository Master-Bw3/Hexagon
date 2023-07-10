pub mod mishap;
pub mod ops;
pub mod state;

use crate::{
    compiler::ops::{compile_op_copy, compile_op_embed, compile_op_push, compile_op_store},
    interpreter::{
        ops::{embed, push, store, EmbedType},
        state::StackExt,
    },
    iota::{EntityIota, EntityType, Iota, PatternIota, Signature, SignatureExt},
    parse_config::Config,
    parser::{ActionValue, AstNode, OpName, OpValue},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{
    mishap::Mishap,
    state::{Holding, State},
};

pub fn interpret(node: AstNode, config: Option<Config>) -> Result<State, (Mishap, (usize, usize))> {
    let mut state = State::default();
    let pattern_registry = PatternRegistry::construct();
    state.ravenmind = Some(Iota::List(vec![]));

    if let Some(conf) = config {
        state.entities = conf.entities;
        state.libraries = conf.libraries;
    }

    //if caster is not overriden by config then set default caster values
    match state.entities.get("Caster") {
        Some(_) => (),
        None => {
            state.entities.insert(
                "Caster".to_string(),
                EntityIota {
                    name: "Caster".to_string(),
                    entity_type: EntityType::Player,
                    holding: Box::new(Holding::None),
                },
            );
        }
    }

    (interpret_node(node, &mut state, &pattern_registry)).map(|state| state.clone())
}

fn interpret_node<'a>(
    node: AstNode,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, (Mishap, (usize, usize))> {
    // println!("a: {:?}, {:?}", state.stack, state.buffer);

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

        AstNode::Action { name, value, line } => {
            interpret_action(name, value, state, pattern_registry).map_err(|err| (err, line))
        }
        AstNode::Hex(nodes) => {
            interpret_action("open_paren".to_string(), None, state, pattern_registry)
                .map_err(|err| (err, (0, 0)))?;
            for node in nodes {
                interpret_node(node, state, pattern_registry)?;
            }
            interpret_action("close_paren".to_string(), None, state, pattern_registry)
                .map_err(|err| (err, (0, 0)))?;

            Ok(state)
        }
        AstNode::Op { name, arg, line } => {
            interpret_op(name, arg, state, pattern_registry).map_err(|err| (err, (0, 0)))
        }
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
            line,
        } => {
            if state.consider_next {
                return Err((Mishap::OpCannotBeConsidered, line));
            }

            if state.buffer.is_some() {
                //push patterns to buffer
                if let AstNode::Hex(nodes) = *condition {
                    for node in nodes {
                        interpret_node(node, state, pattern_registry)?;
                    }
                }
                //push success hex to buffer
                interpret_node(*succeed, state, pattern_registry)?;

                //push fail hex to buffer (if there is one)
                match fail {
                    Some(fail_node) => {
                        interpret_node(*fail_node, state, pattern_registry)?;
                    }
                    None => {
                        interpret_node(AstNode::Hex(vec![]), state, pattern_registry)?;
                    }
                }
                //push augur's to buffer
                push_pattern("if".to_string(), None, state, pattern_registry, false);
            } else {
                interpret_node(*condition, state, pattern_registry)?;

                let condition = state.stack.get_bool(0, 1).map_err(|err| (err, line))?;

                state.stack.remove_args(&1);

                if condition {
                    interpret_node(*succeed, state, pattern_registry)?;
                } else if let Some(node) = fail {
                    interpret_node(*node, state, pattern_registry)?;
                }
            }
            Ok(state)
        }
    }
}

pub fn interpret_op<'a>(
    name: OpName,
    arg: Option<OpValue>,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    if state.consider_next {
        return Err(Mishap::OpCannotBeConsidered);
    }

    if state.buffer.is_some() {
        let compiled = match name {
            crate::parser::OpName::Store => {
                compile_op_store(&mut state.heap, pattern_registry, &arg)
            }
            crate::parser::OpName::Copy => compile_op_copy(&mut state.heap, pattern_registry, &arg),
            crate::parser::OpName::Push => compile_op_push(&mut state.heap, pattern_registry, &arg),
            crate::parser::OpName::Embed => {
                compile_op_embed(pattern_registry, &state.buffer, &arg, EmbedType::Normal)
            }
            crate::parser::OpName::SmartEmbed => {
                compile_op_embed(pattern_registry, &state.buffer, &arg, EmbedType::Smart)
            }
            crate::parser::OpName::ConsiderEmbed => {
                compile_op_embed(pattern_registry, &state.buffer, &arg, EmbedType::Consider)
            }
            crate::parser::OpName::IntroEmbed => {
                compile_op_embed(pattern_registry, &state.buffer, &arg, EmbedType::IntroRetro)
            }
        }?;
        for iota in compiled {
            push_iota(iota, state, false)
        }
    } else {
        match name {
            crate::parser::OpName::Store => store(&arg, state, false),
            crate::parser::OpName::Copy => store(&arg, state, true),
            crate::parser::OpName::Push => push(&arg, state),
            crate::parser::OpName::Embed => embed(&arg, state, pattern_registry, EmbedType::Normal),
            crate::parser::OpName::SmartEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::Smart)
            }
            crate::parser::OpName::ConsiderEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::Consider)
            }
            crate::parser::OpName::IntroEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::IntroRetro)
            }
        }?;
    }

    Ok(state)
}

pub fn interpret_action<'a>(
    name: String,
    value: Option<ActionValue>,
    mut state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let pattern = pattern_registry
        .find(&name, &value)
        .ok_or(Mishap::InvalidPattern)?;

    let is_escape = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "escape", &None);

    let is_retro = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "close_paren", &None);

    if state.consider_next {
        push_pattern(name, value, state, pattern_registry, true);
        state.consider_next = false;
        return Ok(state);
    }

    if state.buffer.is_some() && !(is_escape || is_retro) {
        push_pattern(name, value, state, pattern_registry, false);
        return Ok(state);
    }

    pattern.operate(state, pattern_registry, &value)?;

    Ok(state)
}

pub fn push_pattern(
    pattern: String,
    value: Option<ActionValue>,
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
