use im::Vector;

#[derive(Debug, Clone)]
pub enum ContinuationFrame {
    Evaluate(FrameEvaluate),
    EndEval(FrameEndEval),
    ForEach(FrameForEach),
}

impl ContinuationFrameTrait for ContinuationFrame {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
        macros: &Macros,

    ) -> Result<(), (Mishap, (usize, usize))> {
        match self {
            ContinuationFrame::Evaluate(frame) => frame.evaluate(state, pattern_registry, macros),
            ContinuationFrame::EndEval(frame) => frame.evaluate(state, pattern_registry, macros),
            ContinuationFrame::ForEach(frame) => frame.evaluate(state, pattern_registry, macros),
        }
    }

    fn break_out(&self, state: &mut State) -> bool {
        match self {
            ContinuationFrame::Evaluate(frame) => frame.break_out(state),
            ContinuationFrame::EndEval(frame) => frame.break_out(state),
            ContinuationFrame::ForEach(frame) => frame.break_out(state),
        }
    }
}

use crate::{
    iota::{
        hex_casting::pattern::{PatternIota, SignatureExt},
        Iota,
    },
    parser::{AstNode, OpName, OpValue, Macros},
    pattern_registry::PatternRegistry,
};
use std::{cell::RefCell, rc::Rc, collections::HashMap};

use super::{interpret_node, mishap::Mishap, state::State};

pub type Continuation = Vector<ContinuationFrame>;

pub trait ContinuationFrameTrait: std::fmt::Debug {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
        macros: &Macros,
    ) -> Result<(), (Mishap, (usize, usize))>;

    fn break_out(&self, state: &mut State) -> bool;
}

#[derive(Clone, Debug)]
pub struct FrameEvaluate {
    pub nodes_queue: Vector<AstNode>,
}

impl ContinuationFrameTrait for FrameEvaluate {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
        macros: &Macros,
    ) -> Result<(), (Mishap, (usize, usize))> {
        let mut new_frame = self.clone();
        let node = new_frame.nodes_queue.pop_front();

        match node {
            //if there are still nodes left in the frame:
            Some(n) => {
                //push a new frame to the continuation containing the rest of this frame
                state
                    .continuation
                    .push_back(ContinuationFrame::Evaluate(new_frame));

                interpret_node(n.clone(), state, pattern_registry, macros,)?;
                Ok(())
            }
            //else, don't push any new frames
            None => Ok(()),
        }
    }

    fn break_out(&self, state: &mut State) -> bool {
        state.continuation.pop_back();
        false
    }
}

#[derive(Clone, Debug)]
pub struct FrameEndEval {}

impl ContinuationFrameTrait for FrameEndEval {
    fn evaluate(
        &self,
        state: &mut State,
        _: &PatternRegistry,
        macros: &Macros,

    ) -> Result<(), (Mishap, (usize, usize))> {
        state.consider_next = false;
        Ok(())
    }

    fn break_out(&self, _state: &mut State) -> bool {
        true
    }
}

type ThothAcc = Rc<RefCell<Vector<Rc<dyn Iota>>>>;

#[derive(Clone, Debug)]
pub struct FrameForEach {
    pub data: Vector<Rc<dyn Iota>>,
    pub code: Vector<AstNode>,
    pub base_stack: Option<Vector<Rc<dyn Iota>>>,
    pub acc: ThothAcc,
}

impl ContinuationFrameTrait for FrameForEach {
    fn evaluate(
        &self,
        state: &mut State,
        _: &PatternRegistry,
        macros: &Macros,

    ) -> Result<(), (Mishap, (usize, usize))> {
        let stack = match &self.base_stack {
            //thoth entry point
            None => state.stack.clone(),

            //thoth iteration
            Some(base) => {
                self.acc.borrow_mut().append(state.stack.clone());
                base.clone()
            }
        };

        let stack_top = if !self.data.is_empty() {
            let mut new_data = self.data.clone();
            let top = new_data.pop_front().unwrap();

            state
                .continuation
                .push_back(ContinuationFrame::ForEach(FrameForEach {
                    data: new_data,
                    code: self.code.clone(),
                    base_stack: Some(stack.clone()),
                    acc: self.acc.clone(),
                }));

            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: self.code.clone(),
                }));

            top
        } else {
            Rc::new(self.acc.borrow().clone())
        };

        state.stack = stack.into_iter().collect();
        state.stack.push_back(stack_top);

        Ok(())
    }

    fn break_out(&self, state: &mut State) -> bool {
        state.continuation.pop_back();

        let mut new_stack = self.base_stack.clone().unwrap_or(Vector::new());

        let new_acc = self.acc.clone();
        new_acc.borrow_mut().append(state.stack.clone());
        new_stack.push_back(Rc::new(new_acc.borrow().clone()));
        state.stack = new_stack.into_iter().collect();
        true
    }
}

pub fn iota_list_to_ast_node_list(list: Rc<Vector<Rc<dyn Iota>>>) -> Vector<AstNode> {
    list.iter()
        .enumerate()
        .map(
            |(index, iota)| match iota.clone().downcast_rc::<PatternIota>() {
                Ok(pattern) => AstNode::Action {
                    line: pattern.line.unwrap_or((1, 0)),
                    name: pattern.signature.as_str(),
                    value: *pattern.value.clone(),
                },
                Err(_) => AstNode::Op {
                    line: (1, 0),
                    name: OpName::Embed,
                    arg: Some(OpValue::Iota(iota.clone())),
                },
            },
        )
        .collect()
}
