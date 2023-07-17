use std::collections::HashMap;

use crate::{
    interpreter::{
        mishap::{self, Mishap},
        ops::EmbedType,
    },
    iota::{Iota, PatternIota, SignatureExt},
    parse_config::Config,
    parser::{AstNode, OpName},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::{
    if_block::compile_if_block,
    ops::{compile_op_copy, compile_op_embed, compile_op_push, compile_op_store},
};

pub mod if_block;
pub mod ops;

pub fn compile_to_iotas(
    node: AstNode,
    config: &Option<&Config>,
) -> Result<Vec<Iota>, (Mishap, (usize, usize))> {
    let mut heap: HashMap<String, i32> = HashMap::new();

    let great_sigs = config.map_or_else(PatternRegistry::gen_default_great_sigs, |conf| {
        conf.great_spell_sigs.clone()
    });

    let pattern_registry = PatternRegistry::construct(&great_sigs);

    compile_node(&node, &mut heap, 0, &pattern_registry)
}

fn compile_node(
    node: &AstNode,
    heap: &mut HashMap<String, i32>,
    depth: u32,
    pattern_registry: &PatternRegistry,
) -> Result<Vec<Iota>, (Mishap, (usize, usize))> {
    match node {
        AstNode::File(file) => {
            let mut result = vec![];
            for node in file {
                result.append(&mut compile_node(node, heap, depth, pattern_registry)?)
            }
            Ok(result)
        }

        AstNode::Action { line, name, value } => Ok(vec![Iota::Pattern({
            let pattern = pattern_registry
                .find(&name, &value)
                .ok_or((Mishap::InvalidPattern, *line))?;

            //remove output values used by the interpreter
            let new_value = if pattern.internal_name == "number" || pattern.internal_name == "mask"
            {
                value.clone()
            } else {
                None
            };
            PatternIota::from_sig(&pattern.signature, new_value)
        })]),

        AstNode::Hex(hex) => compile_hex_node(hex, heap, depth, pattern_registry),

        AstNode::Op { line, name, arg } => match name {
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
        .map_err(|mishap| (mishap, *line)),

        AstNode::IfBlock {
            line,
            condition,
            succeed,
            fail,
        } => compile_if_block(
            line,
            condition,
            succeed,
            fail,
            depth,
            heap,
            pattern_registry,
        ),
    }
}

fn compile_hex_node(
    hex: &Vec<AstNode>,
    heap: &mut HashMap<String, i32>,
    mut depth: u32,
    pattern_registry: &PatternRegistry,
) -> Result<Vec<Iota>, (Mishap, (usize, usize))> {
    depth += 1;

    let mut result = vec![];

    let mut inner = vec![];
    for node in hex {
        inner.append(&mut compile_node(node, heap, depth, pattern_registry)?)
    }

    result.push(Iota::Pattern(
        PatternIota::from_name(pattern_registry, "open_paren", None).unwrap(),
    ));

    result.append(&mut inner);

    result.push(Iota::Pattern(
        PatternIota::from_name(pattern_registry, "close_paren", None).unwrap(),
    ));

    Ok(result)
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
