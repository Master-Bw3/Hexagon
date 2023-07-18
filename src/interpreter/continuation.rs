use crate::{
    parser::AstNode,
    pattern_registry::{self, PatternRegistry},
};
use std::{
    fmt::{self, Debug},
    rc::Rc,
};

use super::{interpret_node, mishap::Mishap, state::State};

pub type Continuation = Vec<Rc<dyn ContinuationFrame>>;

pub trait ContinuationFrame: Debug {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
    ) -> Result<(), (Mishap, (usize, usize))>;

    fn is_end_eval(&self) -> bool;
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

    fn is_end_eval(&self) -> bool {
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

    fn is_end_eval(&self) -> bool {
        true
    }
}
