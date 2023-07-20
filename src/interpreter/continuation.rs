use im::Vector;

use crate::{
    iota::{
        hex_casting::pattern::{PatternIota, SignatureExt},
        Iota,
    },
    parser::{AstNode, OpName, OpValue}, pattern_registry::PatternRegistry,
};
use std::rc::Rc;

use super::{interpret_node, mishap::Mishap, state::State};

pub type Continuation = Vec<Rc<dyn ContinuationFrame>>;

pub trait ContinuationFrame: std::fmt::Debug {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
    ) -> Result<(), (Mishap, (usize, usize))>;

    fn break_out(&self, state: &mut State) -> bool;
}

#[derive(Clone, Debug)]
pub struct FrameEvaluate {
    pub nodes: Vec<AstNode>,
}

impl ContinuationFrame for FrameEvaluate {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
    ) -> Result<(), (Mishap, (usize, usize))> {
        let mut new_frame = self.clone();
        let node = new_frame.nodes.pop();

        match node {
            //if there are still nodes left in the frame:
            Some(n) => {
                //push a new frame to the continuation containing the rest of this frame
                state.continuation.push(Rc::new(new_frame));

                interpret_node(n.clone(), state, pattern_registry)?;
                Ok(())
            }
            //else, don't push any new frames
            None => Ok(()),
        }
    }

    fn break_out(&self, state: &mut State) -> bool {
        state.continuation.pop();
        false
    }
}

#[derive(Clone, Debug)]
pub struct FrameEndEval {}

impl ContinuationFrame for FrameEndEval {
    fn evaluate(
        &self,
        state: &mut State,
        _: &PatternRegistry,
    ) -> Result<(), (Mishap, (usize, usize))> {
        state.consider_next = false;
        Ok(())
    }

    fn break_out(&self, _state: &mut State) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct FrameForEach {
    pub data: Vec<Rc<dyn Iota>>,
    pub code: Vec<AstNode>,
    pub base_stack: Option<Vector<Rc<dyn Iota>>>,
    pub acc: Vector<Rc<dyn Iota>>,
}

impl ContinuationFrame for FrameForEach {
    fn evaluate(
        &self,
        state: &mut State,
        _: &PatternRegistry,
    ) -> Result<(), (Mishap, (usize, usize))> {
        let (stack, new_acc) = match &self.base_stack {
            //thoth entry point
            None => (Vector::from(state.stack.clone()), self.acc.clone()),

            //thoth iteration
            Some(base) => (base.clone(), {
                let mut new_acc = self.acc.clone();
                new_acc.append(Vector::from(state.stack.clone()));
                new_acc
            }),
        };

        let stack_top = if !self.data.is_empty() {
            let mut new_data = self.data.clone();
            let top = new_data.pop().unwrap();

            state.continuation.push(Rc::new(FrameForEach {
                data: new_data,
                code: self.code.clone(),
                base_stack: Some(stack.clone()),
                acc: new_acc,
            }));

            state.continuation.push(Rc::new(FrameEvaluate {
                nodes: self.code.clone(),
            }));

            top
        } else {
            Rc::new(new_acc)
        };

        state.stack = stack.into_iter().collect();
        state.stack.push(stack_top);

        Ok(())
    }

    fn break_out(&self, state: &mut State) -> bool {
        state.continuation.pop();

        let mut new_stack = self.base_stack.clone().unwrap_or(Vector::new());

        let mut new_acc = self.acc.clone();
        new_acc.append(Vector::from(state.stack.clone()));
        new_stack.push_back(Rc::new(new_acc));
        state.stack = new_stack.into_iter().collect();
        true
    }
}

pub fn iota_list_to_ast_node_list(list: &[Rc<dyn Iota>]) -> Vec<AstNode> {
    list.iter()
        .rev()
        .enumerate()
        .map(|(index, iota)| match iota.clone().downcast_rc::<PatternIota>() {
            Ok(pattern) => AstNode::Action {
                line: (index + 1, 0),
                name: pattern.signature.as_str(),
                value: *pattern.value.clone(),
            },
            Err(_) => AstNode::Op {
                line: (index, 0),
                name: OpName::Embed,
                arg: Some(OpValue::Iota(iota.clone())),
            },
        })
        .collect()
}
