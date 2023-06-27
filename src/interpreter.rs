use std::collections::HashMap;

use crate::{iota::Iota, parser::AstNode};

struct State {
    stack: Vec<Iota>,
    ravenmind: Option<Iota>,
}

fn interpret(node: AstNode) -> Vec<Iota> {
    let mut state = State {
        stack: vec![],
        ravenmind: None,
    };
    let mut heap: HashMap<String, i32> = HashMap::new();

    match node {
        AstNode::Action { name, value } => {
            let new_stack = vec![];//find action -> operate()

            state.stack = new_stack;
        }
        AstNode::Hex(_) => todo!(),
        AstNode::Op { name, arg } => todo!(),
        AstNode::IfBlock {
            condition,
            succeed,
            fail,
        } => todo!(),
    }
    state.stack
}
