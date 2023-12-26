pub mod continuation;
pub mod error;
pub mod mishap;
pub mod ops;
pub mod state;

use std::{rc::Rc, time::Duration};

use im::Vector;

use crate::{
    compiler::{
        compile_node,
        if_block::compile_if_block,
        ops::{compile_op_copy, compile_op_embed, compile_op_push, compile_op_store},
        while_block::{compile_do_while_block, compile_while_block},
    },
    interpreter::ops::{embed, push, store, EmbedType},
    iota::{
        hex_casting::{
            null::NullIota,
            pattern::{PatternIota, Signature, SignatureExt},
        },
        Iota,
    },
    parse_config::Config,
    parser::{ActionValue, AstNode, Location, Macros, OpName, OpValue},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{
    continuation::{
        iota_list_to_ast_node_list, ContinuationFrame, ContinuationFrameTrait, FrameEvaluate,
    },
    error::print_interpreter_error,
    mishap::Mishap,
    state::{Considered, Entity, EntityType, Holding, State},
};

pub fn interpret(
    node: AstNode,
    config: &Config,
    macros: Macros,
    source: &str,
    source_path: &str,
) -> Result<State, (Mishap, Location)> {
    let mut state = State {
        ..Default::default()
    };
    state.entities = config.entities.clone();
    state.libraries = config.libraries.clone();

    let pattern_registry = PatternRegistry::construct(&config.great_spell_sigs);

    //compile to get heap size so that the ravenmind can be set to the right lenght
    //TODO: replace this with a thing that just looks for var nodes and counts them or something
    compile_node(&node, &mut state.heap, 0, &pattern_registry, &macros).unwrap();
    let null: Rc<dyn Iota> = Rc::new(NullIota);
    state.ravenmind = Some(Rc::new(Vector::from(vec![null; state.heap.keys().len()])));

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

    (run_vm(
        node,
        &mut state,
        &pattern_registry,
        &macros,
        source,
        source_path,
    ))
    .map(|state| state.clone())
}

fn run_vm<'a>(
    node: AstNode,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
    source: &str,
    source_path: &str,
) -> Result<&'a mut State, (Mishap, Location)> {
    match node {
        AstNode::Program(nodes) => {
            //initialize the vm
            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: Vector::from(nodes),
                }));

            //loop through every frame until there aren't any more
            while !state.continuation.is_empty() {
                //get top frame and remove it from the stack
                let frame = state.continuation.pop_back().unwrap();

                //evaluate the top frame (mutates state)
                frame.evaluate(state, pattern_registry, macros)?;
            }

            while !state.wisps.is_empty() {
                std::thread::sleep(Duration::from_millis(50));
                //technically this means that a destroyed wisp can still execute once more in some situations
                for (name, wisp) in state.wisps.clone().iter() {
                    let result = wisp.evaluate(state, pattern_registry, macros);
                    match result {
                        Ok(wisp) => {
                            state.wisps.insert(name.clone(), wisp);
                        }
                        Err(err) => {
                            print_interpreter_error(err, source, source_path);
                            state.wisps.remove(name);
                        }
                    }
                }
            }

            Ok(state)
        }
        _ => unreachable!(),
    }
}

fn interpret_node<'a>(
    node: AstNode,
    state: &'a mut State,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> Result<&'a mut State, (Mishap, Location)> {
    // println!("a: {:?}, {:?}", state.stack, state.buffer);
    match node {
        AstNode::Action {
            name,
            value,
            location,
        } => interpret_action(name, value, state, pattern_registry, &macros, location),
        AstNode::Block { external, nodes } => {
            if external {
                let result = vec![
                    ("open_paren", None),
                    ("read/local", None),
                    ("const/null", None),
                    ("equals", None),
                    ("open_paren", None),
                    ("open_paren", None),
                    ("mask", Some(ActionValue::Bookkeeper("-".to_string()))),
                    ("close_paren", None),
                    ("splat", None),
                    ("write/local", None),
                    ("close_paren", None),
                    ("empty_list", None),
                    ("if", None),
                    ("eval", None),
                    ("close_paren", None),
                    ("number", Some(ActionValue::Iota(Rc::new(5.0)))),
                    ("read/local", None),
                    ("modify_in_place", None),
                ];

                for (name, value) in result {
                    interpret_action(
                        name.to_string(),
                        value,
                        state,
                        &pattern_registry,
                        &macros,
                        Location::Unknown,
                    )?;
                }
            }

            interpret_action(
                "open_paren".to_string(),
                None,
                state,
                pattern_registry,
                &macros,
                Location::Unknown,
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
                Location::Unknown,
            )?;

            //combine external with rest of hex
            if external {
                interpret_action(
                    "concat".to_string(),
                    None,
                    state,
                    pattern_registry,
                    &macros,
                    Location::Unknown,
                )?;
            }

            Ok(state)
        }
        AstNode::Op {
            name,
            arg,
            location,
        } => {
            interpret_op(name, arg, state, pattern_registry, macros).map_err(|err| (err, location))
        }
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
            location,
        } => {
            if state.consider_next {
                return Err((Mishap::OpCannotBeConsidered, location));
            }

            let compiled = Vector::from(compile_if_block(
                &location,
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
        AstNode::WhileBlock {
            location,
            condition,
            block,
            do_while,
        } => {
            if state.consider_next {
                return Err((Mishap::OpCannotBeConsidered, location));
            }

            let compiled = if do_while {
                Vector::from(compile_do_while_block(
                    &location,
                    &condition,
                    &block,
                    calc_buffer_depth(pattern_registry, &state.buffer),
                    &mut state.heap,
                    pattern_registry,
                    macros,
                )?)
            } else {
                Vector::from(compile_while_block(
                    &location,
                    &condition,
                    &block,
                    calc_buffer_depth(pattern_registry, &state.buffer),
                    &mut state.heap,
                    pattern_registry,
                    macros,
                )?)
            };

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
        AstNode::Program(_) => unreachable!(),
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
            OpName::Store => compile_op_store(&mut state.heap, pattern_registry, &arg),
            OpName::Copy => compile_op_copy(&mut state.heap, pattern_registry, &arg),
            OpName::Push => compile_op_push(&mut state.heap, pattern_registry, &arg),
            OpName::Embed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::Normal,
            ),
            OpName::SmartEmbed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::Smart,
            ),
            OpName::ConsiderEmbed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::Consider,
            ),
            OpName::IntroEmbed => compile_op_embed(
                pattern_registry,
                calc_buffer_depth(pattern_registry, &state.buffer),
                &arg,
                EmbedType::IntroRetro,
            ),
            OpName::Init => todo!(),
        }?;
        for iota in compiled {
            push_iota(iota, state, false)
        }
    } else {
        match name {
            OpName::Store => store(&arg, state, false),
            OpName::Copy => store(&arg, state, true),
            OpName::Push => push(&arg, state),
            OpName::Embed => embed(&arg, state, pattern_registry, EmbedType::Normal, macros),
            OpName::SmartEmbed => embed(&arg, state, pattern_registry, EmbedType::Smart, macros),
            OpName::ConsiderEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::Consider, macros)
            }
            OpName::IntroEmbed => {
                embed(&arg, state, pattern_registry, EmbedType::IntroRetro, macros)
            }
            OpName::Init => todo!(),
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
    location: Location,
) -> Result<&'a mut State, (Mishap, Location)> {
    if let Some((_, AstNode::Block { external: _, nodes })) = macros.get(&name) {
        //check for macro and apply it
        if let Some(ref mut buffer) = state.buffer {
            let compiled = compile_node(
                &AstNode::Program(nodes.clone()),
                &mut state.heap,
                calc_buffer_depth(&pattern_registry, &Some(buffer.clone())),
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
            let mut new_frame = Vector::from(nodes);
            new_frame.append(eval_frame.nodes_queue);
            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: new_frame,
                }));
            return Ok(state);
        }
    }

    let patterns = pattern_registry.find_all(&name, &value);
    if patterns.is_empty() {
        return Err((Mishap::InvalidPattern, location));
    }
    let signature = &patterns[0].signature;

    let is_escape = Signature::from_sig(signature)
        == Signature::from_name(pattern_registry, "escape", &None).unwrap();
    let is_retro = Signature::from_sig(signature)
        == Signature::from_name(pattern_registry, "close_paren", &None).unwrap();

    if state.consider_next {
        push_pattern(name, value, state, pattern_registry, true, location);
        state.consider_next = false;
        return Ok(state);
    }

    if state.buffer.is_some() && !(is_escape || is_retro) {
        push_pattern(name, value, state, pattern_registry, false, location);
        return Ok(state);
    }

    let mut result = Ok(());
    for pattern in patterns {
        let operation_result = pattern.operate(state, pattern_registry, &value);
        match operation_result {
            Ok(_) => {
                result = Ok(());
                break;
            }
            Err(Mishap::IncorrectIota(_, _, _)) | Err(Mishap::NotEnoughIotas(_, _)) => {
                result = operation_result.map(|_| ());
            }
            Err(_) => {
                result = operation_result.map(|_| ());
                break;
            }
        }
    }

    result.map(|_| state).map_err(|mishap| (mishap, location))
}

pub fn push_pattern(
    pattern: String,
    value: Option<ActionValue>,
    state: &mut State,
    pattern_registry: &PatternRegistry,
    considered: bool,
    location: Location,
) {
    push_iota(
        Rc::new(PatternIota::from_name(pattern_registry, &pattern, value, location).unwrap()),
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
    let intro_pattern =
        PatternIota::from_name(registry, "open_paren", None, Location::Unknown).unwrap();
    let retro_pattern =
        PatternIota::from_name(registry, "close_paren", None, Location::Unknown).unwrap();

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
