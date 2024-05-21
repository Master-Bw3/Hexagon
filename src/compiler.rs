use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::{mishap::Mishap, ops::EmbedType},
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{ActionValue, AstNode, Location, Macros, OpName},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{
    external::compile_external,
    if_block::compile_if_block,
    init_heap::init_heap,
    ops::{compile_op_copy, compile_op_embed, compile_op_init, compile_op_push, compile_op_store},
    while_block::{compile_do_while_block, compile_while_block},
};

pub mod external;
pub mod if_block;
pub mod init_heap;
pub mod nbt;
pub mod ops;
pub mod while_block;

pub fn compile_to_iotas(
    node: &AstNode,
    heap: Option<&mut HashMap<String, i32>>,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> CompileResult {
    let mut empty_heap = HashMap::new();
    let mut heap = heap.unwrap_or(&mut empty_heap);
    let result = compile_node(&node, &mut heap, 0, pattern_registry, macros);

    //prepend heap init (sets the size of the ravenmind list)
    result.map(|ref mut x| {
        let mut iotas = init_heap(heap, pattern_registry).unwrap();
        iotas.append(x);
        iotas
    })
}

pub fn compile_node(
    node: &AstNode,
    heap: &mut HashMap<String, i32>,
    depth: u32,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> CompileResult {
    match node {
        AstNode::Program(file) => {
            let mut result = vec![];
            for node in file {
                result.append(&mut compile_node(
                    node,
                    heap,
                    depth,
                    pattern_registry,
                    macros,
                )?)
            }
            Ok(result)
        }

        AstNode::Action {
            location,
            name,
            value,
        } => {
            if let Some((_, AstNode::Block { external: _, nodes })) = macros.get(name) {
                compile_node(
                    &AstNode::Program(nodes.clone()),
                    heap,
                    depth,
                    pattern_registry,
                    macros,
                )
            } else {
                Ok(vec![{
                    let pattern = pattern_registry
                        .find(name, value)
                        .ok_or((Mishap::InvalidPattern, location.clone(), name.clone()))?;

                    //remove output values used by the interpreter
                    //once signature generation exists for number, all values can be ignored
                    let new_value =
                        if pattern.internal_name == "number" || pattern.internal_name == "mask" {
                            value.clone()
                        } else {
                            value.clone()
                        };
                    Rc::new(PatternIota::from_sig(
                        &pattern.signature,
                        new_value,
                        location.clone(),
                    ))
                }])
            }
        }

        AstNode::Block { external, nodes } => {
            let block_heap = &mut heap.clone();
            let result = compile_hex_node(nodes, block_heap, depth, pattern_registry, macros)
                .and_then(|mut x| {
                    x.append(&mut block_end(heap.len(), pattern_registry));
                    Ok(x)
                });
            

            for key in heap.keys().cloned().collect::<Vec<String>>() {
                let new_val = block_heap[&key];
                heap.insert(key, new_val);
            }

            if *external {
                result.map(|ref mut x| compile_external(x, pattern_registry))
            } else {
                result
            }
        }

        AstNode::Op {
            location,
            name,
            arg,
        } => match name {
            OpName::Init => compile_op_init(heap, pattern_registry, arg),
            OpName::Store => compile_op_store(heap, pattern_registry, arg),
            OpName::Copy => compile_op_copy(heap, pattern_registry, arg),
            OpName::Push => compile_op_push(heap, pattern_registry, arg),
            OpName::Embed => compile_op_embed(pattern_registry, depth, arg, EmbedType::Normal),
            OpName::SmartEmbed => compile_op_embed(pattern_registry, depth, arg, EmbedType::Smart),
            OpName::IntroEmbed => {
                compile_op_embed(pattern_registry, depth, arg, EmbedType::IntroRetro)
            }
            OpName::ConsiderEmbed => {
                compile_op_embed(pattern_registry, depth, arg, EmbedType::Consider)
            }
        }
        .map_err(|mishap| (mishap, location.clone(), name.to_string())),

        AstNode::IfBlock {
            location,
            condition,
            succeed,
            fail,
        } => compile_if_block(
            location,
            condition,
            succeed,
            fail,
            depth,
            heap,
            pattern_registry,
            macros,
        ),
        AstNode::WhileBlock {
            location,
            condition,
            block,
            do_while,
        } => {
            if *do_while {
                compile_do_while_block(
                    location,
                    condition,
                    block,
                    depth,
                    heap,
                    pattern_registry,
                    macros,
                )
            } else {
                compile_while_block(
                    location,
                    condition,
                    block,
                    depth,
                    heap,
                    pattern_registry,
                    macros,
                )
            }
        }
    }
}

pub type CompileResult = Result<Vec<Rc<dyn Iota>>, (Mishap, Location, String)>;

fn compile_hex_node(
    hex: &Vec<AstNode>,
    heap: &mut HashMap<String, i32>,
    mut depth: u32,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> CompileResult {
    depth += 1;

    let mut result: Vec<Rc<dyn Iota>> = vec![];

    let mut inner = vec![];
    for node in hex {
        inner.append(&mut compile_node(
            node,
            heap,
            depth,
            pattern_registry,
            macros,
        )?)
    }

    result.push(Rc::new(
        PatternIota::from_name(pattern_registry, "open_paren", None, Location::Unknown).unwrap(),
    ));

    result.append(&mut inner);

    result.push(Rc::new(
        PatternIota::from_name(pattern_registry, "close_paren", None, Location::Unknown).unwrap(),
    ));

    Ok(result)
}

fn block_end<'a>(heap_len: usize, registry: &'a PatternRegistry) -> Vec<Rc<dyn Iota>> {
    vec![
        Rc::new(PatternIota::from_name(registry, "read/local", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(0.0))),
                Location::Unknown,
            )
            .unwrap(),
        ),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(heap_len as f64))),
                Location::Unknown,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "slice", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "write/local", None, Location::Unknown).unwrap()),
    ]
}

pub fn wrap_pattern(pat: PatternIota) -> Rc<dyn Iota> {
    Rc::new(pat)
}

// pub fn calc_eval_depth(registry: &PatternRegistry, iotas: &Vec<Iota>) -> u32 {
//     let intro_pattern =
//         Iota::Pattern(PatternIota::from_name(registry, "open_paren", None).unwrap());
//     let retro_pattern =
//         Iota::Pattern(PatternIota::from_name(registry, "close_paren", None).unwrap());

//     let intro_count: u32 = iotas
//         .iter()
//         .fold(0, |acc, x| if x == &intro_pattern { acc + 1 } else { acc });

//     let retro_count: u32 = iotas
//         .iter()
//         .fold(0, |acc, x| if x == &retro_pattern { acc + 1 } else { acc });

//     intro_count - retro_count
// }
