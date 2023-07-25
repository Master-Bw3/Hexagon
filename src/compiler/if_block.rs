use std::{collections::HashMap, rc::Rc};

use crate::{
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{AstNode, Macros},
    pattern_registry::PatternRegistry,
};

use super::{compile_node, CompileResult};

pub fn compile_if_block(
    _line: &(usize, usize),
    condition: &AstNode,
    succeed: &AstNode,
    fail: &Option<Box<AstNode>>,
    depth: u32,
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> CompileResult {
    let mut result: Vec<Rc<dyn Iota>> = vec![];

    //append condition to result
    if let AstNode::Hex(condition_hex) = (*condition).clone() {
        for node in condition_hex {
            result.append(&mut compile_node(
                &node,
                heap,
                depth,
                pattern_registry,
                macros,
            )?)
        }
    };

    //push success hex to result
    result.append(&mut compile_node(
        succeed,
        heap,
        depth,
        pattern_registry,
        macros,
    )?);
    //push fail hex to result (if there is one)
    match fail {
        Some(fail_node) => match **fail_node {
            AstNode::Hex(_) => {
                // "else"
                result.append(&mut compile_node(
                    fail_node,
                    heap,
                    depth,
                    pattern_registry,
                    macros,
                )?);
            }
            // "if else"
            AstNode::IfBlock {
                line: _,
                condition: _,
                succeed: _,
                fail: _,
            } => {
                result.append(&mut compile_node(
                    &AstNode::Hex(vec![(**fail_node).clone()]),
                    heap,
                    depth,
                    pattern_registry,
                    macros,
                )?);
                result.push(Rc::new(
                    PatternIota::from_name(pattern_registry, "eval", None, None).unwrap(),
                ));
            }
            _ => unreachable!(),
        },
        None => {
            result.append(&mut compile_node(&AstNode::Hex(vec![]), heap, depth, pattern_registry, macros)?);
        }
    }
    //push augur's to buffer
    result.push(Rc::new(
        PatternIota::from_name(pattern_registry, "if", None, None).unwrap(),
    ));

    Ok(result)
}
