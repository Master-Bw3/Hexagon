use std::{collections::HashMap, rc::Rc};

use crate::{
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{ActionValue, AstNode, Location, Macros},
    pattern_registry::PatternRegistry,
};

use super::{compile_node, wrap_pattern, CompileResult};

pub fn compile_while_block(
    location: &Location,
    condition: &AstNode,
    block: &AstNode,
    depth: u32,
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    macros: &Macros,
) -> CompileResult {
    let mut result: Vec<Rc<dyn Iota>> = vec![];

    result.push(Rc::new(
        PatternIota::from_name(pattern_registry, "open_paren", None, location.clone()).unwrap(),
    ));

    //push block
    result.append(&mut compile_node(
        block,
        heap,
        depth,
        pattern_registry,
        macros,
    )?);

    result.extend(
        vec![
            PatternIota::from_name(pattern_registry, "open_paren", None, location.clone()),
            PatternIota::from_name(
                pattern_registry,
                "mask",
                Some(ActionValue::Bookkeeper("vv".to_string())),
                location.clone(),
            ),
            PatternIota::from_name(pattern_registry, "close_paren", None, location.clone()),
            PatternIota::from_name(pattern_registry, "swap", None, location.clone()),
            PatternIota::from_name(pattern_registry, "concat", None, location.clone()),
            PatternIota::from_name(pattern_registry, "const/null", None, location.clone()),
            PatternIota::from_name(pattern_registry, "singleton", None, location.clone()),
            PatternIota::from_name(pattern_registry, "for_each", None, location.clone()),
            PatternIota::from_name(
                pattern_registry,
                "number",
                Some(ActionValue::Iota(Rc::new(2.0))),
                location.clone(),
            ),
            PatternIota::from_name(pattern_registry, "last_n_list", None, location.clone()),
            PatternIota::from_name(pattern_registry, "stack_len", None, location.clone()),
            PatternIota::from_name(pattern_registry, "last_n_list", None, location.clone()),
            PatternIota::from_name(pattern_registry, "reverse_list", None, location.clone()),
            PatternIota::from_name(pattern_registry, "deconstruct", None, location.clone()),
            PatternIota::from_name(pattern_registry, "swap", None, location.clone()),
            PatternIota::from_name(
                pattern_registry,
                "mask",
                Some(ActionValue::Bookkeeper("v".to_string())),
                location.clone(),
            ),            PatternIota::from_name(pattern_registry, "splat", None, location.clone()),
            PatternIota::from_name(pattern_registry, "swap", None, location.clone()),
            PatternIota::from_name(pattern_registry, "append", None, location.clone()),
            PatternIota::from_name(pattern_registry, "splat", None, location.clone()),
        ]
        .into_iter()
        .map(Result::unwrap)
        .map(wrap_pattern),
    );

    //append condition to result
    if let AstNode::Block { external: _, nodes } = (*condition).clone() {
        for node in nodes {
            result.append(&mut compile_node(
                &node,
                heap,
                depth,
                pattern_registry,
                macros,
            )?)
        }
    };

    result.extend(
        vec![
            PatternIota::from_name(pattern_registry, "open_paren", None, location.clone()),
            PatternIota::from_name(pattern_registry, "duplicate", None, location.clone()),
            PatternIota::from_name(pattern_registry, "eval", None, location.clone()),
            PatternIota::from_name(pattern_registry, "close_paren", None, location.clone()),
            PatternIota::from_name(pattern_registry, "open_paren", None, location.clone()),
            PatternIota::from_name(
                pattern_registry,
                "mask",
                Some(ActionValue::Bookkeeper("v".to_string())),
                location.clone(),
            ),
            PatternIota::from_name(pattern_registry, "close_paren", None, location.clone()),
            PatternIota::from_name(pattern_registry, "splat", None, location.clone()),
            PatternIota::from_name(pattern_registry, "if", None, location.clone()),
            PatternIota::from_name(pattern_registry, "eval", None, location.clone()),
            PatternIota::from_name(pattern_registry, "close_paren", None, location.clone()),
            PatternIota::from_name(pattern_registry, "duplicate", None, location.clone()),
            PatternIota::from_name(pattern_registry, "eval", None, location.clone()),
        ]
        .into_iter()
        .map(Result::unwrap)
        .map(wrap_pattern),
    );

    Ok(result)
}
