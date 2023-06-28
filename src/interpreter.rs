pub mod mishap;
pub mod state;
mod ops;

use std::collections::HashMap;

use crate::{
    parser::{ActionValue, AstNode},
    pattern_registry::{PatternRegistry, PatternRegistryExt}, interpreter::ops::{store, push},
};

use self::state::State;

pub fn interpret(node: AstNode) -> Result<State, String> {
    let mut state = State {
        stack: vec![],
        ravenmind: None,
    };
    let mut heap: HashMap<String, i32> = HashMap::new();
    let pattern_registry = PatternRegistry::construct();

    (interpret_node(node, &mut state, &mut heap, &pattern_registry)).cloned()
}

fn interpret_node<'a>(
    node: AstNode,
    mut state: &'a mut State,
    heap: &mut HashMap<String, i32>,
    pattern_registry: &'a PatternRegistry,
) -> Result<&'a mut State, String> {
    println!("{:?}, {:?}", state, heap);
    match node {
        AstNode::Action { name, value } => {
            //Push value to stack if there is one. Otherwise, evaluate the pattern
            match value {
                Some(val) => match val {
                    ActionValue::Iota(iota) => {
                        state.stack.push(iota);
                        Ok(state)
                    }
                    ActionValue::Bookkeeper(_) => todo!(),
                },
                None => {
                    let pattern = pattern_registry.find(name).ok_or("Invalid Action")?;

                    pattern
                        .operate(state, value)
                        .map_err(|err: mishap::Mishap| format!("{:?}", err))

                }
            }
        }
        AstNode::Hex(nodes) => {

            for node in nodes {
                state = interpret_node(node, state, heap, pattern_registry)?;
            };
            Ok(state)


        }
        AstNode::Op { name, arg } => {
            match name {
                crate::parser::OpName::Store => store(&arg, state, heap, false),
                crate::parser::OpName::Copy => store(&arg, state, heap, true),
                crate::parser::OpName::Push => push(&arg, state, heap),
                crate::parser::OpName::Embed => todo!(),
                crate::parser::OpName::SmartEmbed => todo!(),
                crate::parser::OpName::ConsiderEmbed => todo!(),
                crate::parser::OpName::IntroEmbed => todo!(),
            }?;

            Ok(state)
        },
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
        } => todo!(),
    }
}
