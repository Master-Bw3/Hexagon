pub mod continuation;
pub mod mishap;
pub mod ops;
pub mod state;

use std::rc::Rc;

use im::{vector, Vector};

use crate::{
    compiler::{
        compile_to_iotas,
        if_block::compile_if_block,
        ops::{compile_op_copy, compile_op_embed, compile_op_push, compile_op_store},
    },
    interpreter::{
        ops::{embed, push, store, EmbedType},
        state::StackExt,
    },
    iota::{
        hex_casting::{
            bool::BooleanIota,
            pattern::{PatternIota, Signature, SignatureExt},
        },
        Iota,
    },
    parse_config::Config,
    parser::{ActionValue, AstNode, Macros, OpName, OpValue},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{
    continuation::{
        iota_list_to_ast_node_list, ContinuationFrame, ContinuationFrameTrait, FrameEvaluate,
    },
    mishap::Mishap,
    state::{Considered, Entity, EntityType, Holding, State},
};

pub fn interpret(
    node: AstNode,
    config: &Config,
    macros: Macros,
) -> Result<State, (Mishap, (usize, usize))> {
    let mut state = State {
        ravenmind: Some(Rc::new(im::vector![])),
        ..Default::default()
    };
    state.entities = config.entities.clone();
    state.libraries = config.libraries.clone();

    let pattern_registry = PatternRegistry::construct(&config.great_spell_sigs);

    //if caster is not overriden by config then set default caster values
    match state.entities.get("Caster") {
        Some(_) => (),
        None => {
            state.entities.insert(
                "Caster".to_string(),
                Entity {
                    name: "Caster".to_string(),
                    entity_type: EntityType::Player,
                    uuid: "[I;0,0,0,0]".to_string(),
                    holding: Box::new(Holding::None),
                },
            );
        }
    }

    (interpret_node(node, &mut state, &pattern_registry, &macros)).map(|state| state.clone())
}

fn interpret_node<'a>(
    node: AstNode,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> Result<&'a mut State, (Mishap, (usize, usize))> {
    // println!("a: {:?}, {:?}", state.stack, state.buffer);

    match node {
        AstNode::File(nodes) => {
            //initialize the vm
            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: Vector::from(nodes),
                }));

            //loop through every frame until there aren't any more
            while !state.continuation.is_empty() {
                //get top fram and remove it from the stack
                let frame = state.continuation.pop_back().unwrap();

                //evaluate the top frame (mutates state)
                frame.evaluate(state, pattern_registry, macros)?;
            }
            Ok(state)
        }

        AstNode::Action { name, value, line } => {
            interpret_action(name, value, state, pattern_registry, &macros, Some(line))
        }
        AstNode::Hex(nodes) => {
            interpret_action(
                "open_paren".to_string(),
                None,
                state,
                pattern_registry,
                &macros,
                None,
            )?;

            for node in nodes {
                interpret_node(node, state, pattern_registry, macros)?;
            }
            interpret_action(
                "close_paren".to_string(),
                None,
                state,
                pattern_registry,
                &macros,
                None,
            )?;

            Ok(state)
        }
        AstNode::Op { name, arg, line } => {
            interpret_op(name, arg, state, pattern_registry, macros).map_err(|err| (err, line))
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

            let compiled = Vector::from(compile_if_block(
                &line,
                &condition,
                &succeed,
                &fail,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &mut state.heap,
                pattern_registry,
                macros,
            )?);

            if let Some(buffer) = &mut state.buffer {
                buffer.append(compiled.iter().map(|x| (x.clone(), false)).collect())
            } else {
                state
                    .continuation
                    .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                        nodes_queue: iota_list_to_ast_node_list(Rc::new(compiled)),
                    }))
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
    macros: &Macros,
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
            crate::parser::OpName::Embed => {
                embed(&arg, state, pattern_registry, EmbedType::Normal, macros)
            }
            crate::parser::OpName::SmartEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::Smart, macros)
            }
            crate::parser::OpName::ConsiderEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::Consider, macros)
            }
            crate::parser::OpName::IntroEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::IntroRetro, macros)
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
    macros: &Macros,
    line: Option<(usize, usize)>,
) -> Result<&'a mut State, (Mishap, (usize, usize))> {
    if let Some((_, AstNode::Hex(macro_hex))) = macros.get(&name) {
        //check for macro and apply it
        if let Some(ref mut buffer) = state.buffer {
            let compiled = compile_to_iotas(
                &AstNode::File(macro_hex.clone()),
                Some(&mut state.heap),
                pattern_registry,
                macros,
            )
            .unwrap()
            .into_iter()
            .map(|x| (x, false))
            .collect::<Vector<_>>();
            buffer.append(compiled);
            return Ok(state);
        } else if let ContinuationFrame::Evaluate(eval_frame) =
            state.continuation.pop_back().unwrap()
        {
            let mut new_frame = Vector::from(macro_hex);
            new_frame.append(eval_frame.nodes_queue);
            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: new_frame,
                }));
            return Ok(state);
        }
    }

    let pattern = pattern_registry
        .find(&name, &value)
        .ok_or((Mishap::InvalidPattern, line.unwrap_or((1, 0))))?;
    let is_escape = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "escape", &None).unwrap();

    let is_retro = Signature::from_sig(&pattern.signature)
        == Signature::from_name(pattern_registry, "close_paren", &None).unwrap();

    if state.consider_next {
        push_pattern(name, value, state, pattern_registry, true, line);
        state.consider_next = false;
        return Ok(state);
    }

    if state.buffer.is_some() && !(is_escape || is_retro) {
        push_pattern(name, value, state, pattern_registry, false, line);
        return Ok(state);
    }

    pattern
        .operate(state, pattern_registry, &value)
        .map_err(|err| (err, line.unwrap_or((1, 0))))?;

    Ok(state)
}

pub fn push_pattern(
    pattern: String,
    value: Option<ActionValue>,
    state: &mut State,
    pattern_registry: &PatternRegistry,
    considered: bool,
    line: Option<(usize, usize)>,
) {
    push_iota(
        Rc::new(PatternIota::from_name(pattern_registry, &pattern, value, line).unwrap()),
        state,
        considered,
    )
}

pub fn push_iota(iota: Rc<dyn Iota>, state: &mut State, considered: bool) {
    match state.buffer {
        Some(ref mut buffer) => buffer.push_back((iota, considered)),
        None => state.stack.push_back(iota),
    }
}

fn calc_buffer_depth(
    registry: &PatternRegistry,
    buffer: &Option<Vector<(Rc<dyn Iota>, Considered)>>,
) -> u32 {
    let intro_pattern = PatternIota::from_name(registry, "open_paren", None, None).unwrap();
    let retro_pattern = PatternIota::from_name(registry, "close_paren", None, None).unwrap();

    let intro_count: u32 = if let Some(inner_buffer) = buffer {
        inner_buffer.iter().fold(0, |acc, x| {
            if x.0.tolerates_other(&intro_pattern) && !x.1 {
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
            if x.0.tolerates_other(&retro_pattern) && !x.1 {
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
