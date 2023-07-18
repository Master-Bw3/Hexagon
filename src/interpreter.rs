pub mod continuation;
pub mod mishap;
pub mod ops;
pub mod state;

use std::{collections::HashMap, rc::Rc};

use crate::{
    compiler::{
        if_block::compile_if_block,
        ops::{compile_op_copy, compile_op_embed, compile_op_push, compile_op_store},
    },
    interpreter::{
        ops::{embed, push, store, EmbedType},
        state::StackExt,
    },
    iota::{Iota, PatternIota, Signature, SignatureExt},
    parse_config::Config,
    parser::{ActionValue, AstNode, Instruction, OpName, OpValue},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{
    continuation::{FrameEndEval, FrameEvaluate},
    mishap::Mishap,
    state::{Considered, Entity, EntityType, Holding, State},
};

pub fn interpret(
    node: AstNode,
    config: &Option<&Config>,
    entities: HashMap<String, Entity>,
) -> Result<State, (Mishap, (usize, usize))> {
    let mut state = State {
        ravenmind: Some(Iota::List(vec![])),
        ..Default::default()
    };
    let great_sigs;

    if let Some(conf) = config {
        state.entities = entities.clone();
        state.libraries = conf.libraries.clone();
        great_sigs = conf.great_spell_sigs.clone();
    } else {
        great_sigs = PatternRegistry::gen_default_great_sigs();
    }

    let pattern_registry = PatternRegistry::construct(&great_sigs);

    //if caster is not overriden by config then set default caster values
    match state.entities.get("Caster") {
        Some(_) => (),
        None => {
            state.entities.insert(
                "Caster".to_string(),
                Entity {
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
        AstNode::File(mut nodes) => {
            //initialize the vm
            nodes.reverse();
            state
                .continuation
                .push(Rc::new(FrameEvaluate { nodes: nodes }));

            //loop through every frame until there aren't any more
            while state.continuation.len() > 0 {
                //get top fram and remove it from the stack
                let frame = state.continuation.pop().unwrap().clone();

                //evaluate the top frame (mutates state)
                frame.evaluate(state, pattern_registry)?;
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
            interpret_op(name, arg, state, pattern_registry).map_err(|err| (err, line))
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

            if let Some(buffer) = &mut state.buffer {
                buffer.append(
                    &mut compile_if_block(
                        &line,
                        &condition,
                        &succeed,
                        &fail,
                        calc_buffer_depth(pattern_registry, &Some(buffer.clone())),
                        &mut state.heap,
                        pattern_registry,
                    )?
                    .iter()
                    .map(|x| (x.clone(), false))
                    .collect(),
                )
            } else {
                if let AstNode::Hex(nodes) = *condition {
                    for node in nodes {
                        interpret_node(node, state, pattern_registry)?;
                    }
                }

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
        AstNode::Instruction(instruction) => match instruction {
            Instruction::MetaEvalEnd => {
                state.consider_next = false;
                Ok(state)
            }
        },
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
            crate::parser::OpName::Embed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::Normal,
            ),
            crate::parser::OpName::SmartEmbed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::Smart,
            ),
            crate::parser::OpName::ConsiderEmbed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::Consider,
            ),
            crate::parser::OpName::IntroEmbed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::IntroRetro,
            ),
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
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
) -> Result<&'a mut State, Mishap> {
    let pattern = pattern_registry
        .find(&name, &value)
        .ok_or(Mishap::InvalidPattern)?;

    let is_escape = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "escape", &None).unwrap();

    let is_retro = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "close_paren", &None).unwrap();

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
        Iota::Pattern(PatternIota::from_name(pattern_registry, &pattern, value).unwrap()),
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

fn calc_buffer_depth(registry: &PatternRegistry, buffer: &Option<Vec<(Iota, Considered)>>) -> u32 {
    let intro_pattern =
        Iota::Pattern(PatternIota::from_name(registry, "open_paren", None).unwrap());
    let retro_pattern =
        Iota::Pattern(PatternIota::from_name(registry, "close_paren", None).unwrap());

    let intro_count: u32 = if let Some(inner_buffer) = buffer {
        inner_buffer.iter().fold(0, |acc, x| {
            if x.0 == intro_pattern && !x.1 {
                acc + 1
            } else {
                acc
            }
        })
    } else {
        0
    } + 1;

    let retro_count: u32 = if let Some(inner_buffer) = buffer {
        inner_buffer.iter().fold(0, |acc, x| {
            if x.0 == retro_pattern && !x.1 {
                acc + 1
            } else {
                acc
            }
        })
    } else {
        0
    };

    intro_count - retro_count
}
