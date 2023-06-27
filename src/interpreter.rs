pub mod mishap;
pub mod state;

use std::collections::HashMap;

use crate::{
    parser::{ActionValue, AstNode},
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::state::State;

pub fn interpret(node: AstNode) -> Result<State, String> {
    let state = State {
        stack: vec![],
        ravenmind: None,
    };
    let heap: HashMap<String, i32> = HashMap::new();
    let pattern_registry = PatternRegistry::construct();

    inerpret_node(node, state, heap, &pattern_registry)
}

fn inerpret_node(
    node: AstNode,
    state: State,
    heap: HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
) -> Result<State, String> {
    println!("{:?}", state);
    match node {
        AstNode::Action { name, value } => {
            //Push value to stack if there is one. Otherwise, evaluate the pattern
            match value {
                Some(val) => match val {
                    ActionValue::Iota(iota) => {
                        let mut temp_state = state.clone();
                        temp_state.stack.push(iota);
                        Ok(temp_state)
                    }
                    ActionValue::Bookkeeper(_) => todo!(),
                },
                None => {
                    let pattern = pattern_registry.find(name).ok_or("Invalid Action")?;

                    pattern
                        .operate(state, value)
                        .map_err(|err| format!("{:?}", err))

                }
            }
        }
        AstNode::Hex(nodes) => {
            let mut temp_state = state.clone();

            for node in nodes {
                temp_state = inerpret_node(node, temp_state, heap.clone(), pattern_registry)?;
            };
            Ok(temp_state)


        }
        AstNode::Op { name, arg } => todo!(),
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
        } => todo!(),
    }
}
