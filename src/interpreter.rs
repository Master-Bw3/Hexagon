pub mod mishap;
pub mod state;

use std::collections::HashMap;

use crate::{
    parser::AstNode,
    pattern_registry::{PatternRegistry, PatternRegistryExt},
};

use self::state::State;

pub fn interpret(node: AstNode) -> Result<State, String> {
    let mut state = State {
        stack: vec![],
        ravenmind: None,
    };
    let mut heap: HashMap<String, i32> = HashMap::new();
    let pattern_registry = PatternRegistry::construct();

    match node {
        AstNode::Action { name, value } => {
            let pattern = pattern_registry.find(name).ok_or("Invalid Action")?;
            state = pattern.operate(state, value).map_err(|err| format!("{:?}", err))?;
        }
        AstNode::Hex(nodes) => {
            for node in nodes {
                interpret(node)?;
            }
        }
        AstNode::Op { name, arg } => todo!(),
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
        } => todo!(),
    }

    Ok(state)
}
