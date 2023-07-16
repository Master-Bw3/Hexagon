use std::collections::HashMap;

use crate::{
    interpreter::mishap::{self, Mishap},
    iota::{Iota, PatternIota},
    parse_config::Config,
    parser::{AstNode, OpName},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::ops::compile_op_store;

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

    compile_node(&node, &mut heap, &pattern_registry)
}

fn compile_node(
    node: &AstNode,
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
) -> Result<Vec<Iota>, (Mishap, (usize, usize))> {
    match node {
        AstNode::File(file) => {
            let mut result = vec![];
            for node in file {
                result.append(&mut compile_node(node, heap, pattern_registry)?)
            }
            Ok(result)
        }
        AstNode::Action { line, name, value } => Ok(vec![Iota::Pattern(
            PatternIota::from_name(pattern_registry, &name, value.clone())
                .map_err(|mishap| (mishap, *line))?,
        )]),
        AstNode::Hex(hex) => compile_hex_node(hex, heap, pattern_registry),
        AstNode::Op { line, name, arg } => match name {
            OpName::Store => {
                compile_op_store(heap, pattern_registry, arg).map_err(|mishap| (mishap, *line))
            }
            OpName::Copy => todo!(),
            OpName::Push => todo!(),
            OpName::Embed => todo!(),
            OpName::SmartEmbed => todo!(),
            OpName::ConsiderEmbed => todo!(),
            OpName::IntroEmbed => todo!(),
        },
        AstNode::IfBlock {
            line,
            condition,
            succeed,
            fail,
        } => todo!(),
    }
}

fn compile_hex_node(
    hex: &Vec<AstNode>,
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
) -> Result<Vec<Iota>, (Mishap, (usize, usize))> {
    let mut result = vec![];

    let mut inner = vec![];
    for node in hex {
        inner.append(&mut compile_node(node, heap, pattern_registry)?)
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
