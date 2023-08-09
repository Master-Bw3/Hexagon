use im::{vector, Vector};

#[derive(Debug, Clone)]
pub enum ContinuationFrame {
    Evaluate(FrameEvaluate),
    EndEval(FrameEndEval),
    ForEach(FrameForEach),
    Iterate(FrameIterate),
    Map(FrameMap),
}

impl ContinuationFrameTrait for ContinuationFrame {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
        macros: &Macros,
    ) -> Result<(), (Mishap, Location)> {
        match self {
            ContinuationFrame::Evaluate(frame) => frame.evaluate(state, pattern_registry, macros),
            ContinuationFrame::EndEval(frame) => frame.evaluate(state, pattern_registry, macros),
            ContinuationFrame::ForEach(frame) => frame.evaluate(state, pattern_registry, macros),
            ContinuationFrame::Iterate(frame) => frame.evaluate(state, pattern_registry, macros),
            ContinuationFrame::Map(frame) => frame.evaluate(state, pattern_registry, macros),
        }
    }

    fn break_out(&self, state: &mut State) -> bool {
        match self {
            ContinuationFrame::Evaluate(frame) => frame.break_out(state),
            ContinuationFrame::EndEval(frame) => frame.break_out(state),
            ContinuationFrame::ForEach(frame) => frame.break_out(state),
            ContinuationFrame::Iterate(frame) => frame.break_out(state),
            ContinuationFrame::Map(frame) => frame.break_out(state),
        }
    }
}

use crate::{
    iota::{
        hex_casting::{
            null::NullIota,
            pattern::{PatternIota, SignatureExt},
        },
        Iota,
    },
    parser::{AstNode, Macros, OpName, OpValue, Location},
    pattern_registry::PatternRegistry,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Not, Range},
    rc::Rc,
};

use super::{interpret_node, mishap::Mishap, state::State};

pub type Continuation = Vector<ContinuationFrame>;

pub trait ContinuationFrameTrait: std::fmt::Debug {
    fn evaluate(
        &self,
        state: &mut State,
        pattern_registry: &PatternRegistry,
        macros: &Macros,
    ) -> Result<(), (Mishap, Location)>;

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
    ) -> Result<(), (Mishap, Location)> {
        let mut new_frame = self.clone();
        let node = new_frame.nodes_queue.pop_front();

        match node {
            //if there are still nodes left in the frame:
            Some(n) => {
                //push a new frame to the continuation containing the rest of this frame
                state
                    .continuation
                    .push_back(ContinuationFrame::Evaluate(new_frame));

                interpret_node(n.clone(), state, pattern_registry, macros)?;
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
    ) -> Result<(), (Mishap, Location)> {
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
    ) -> Result<(), (Mishap, Location)> {
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

#[derive(Debug, Clone)]
pub struct FrameIterate {
    pub base_stack: Option<Vector<Rc<dyn Iota>>>,
    pub index: usize,
    pub collect: (usize, usize),
    pub acc: ThothAcc,
    pub prev: Rc<dyn Iota>,
    pub gen_next_code: Vector<AstNode>,
    pub maps: Vector<Vector<AstNode>>,
    pub collect_single: bool,
}

impl ContinuationFrameTrait for FrameIterate {
    fn evaluate(
        &self,
        state: &mut State,
        _pattern_registry: &PatternRegistry,
        _macros: &Macros,
    ) -> Result<(), (Mishap, Location)> {
        let base_stack = match &self.base_stack {
            //entry point
            None => state.stack.clone(),

            //iteration
            Some(base) => base.clone(),
        };

        let new_acc = self.acc.clone();

        if self.index <= self.collect.1 && self.index >= self.collect.0 {
            //if index in collect range, push top of stack to accumulator
            if self.base_stack.is_none() {
                //on first just push the inital value
                new_acc.borrow_mut().push_back(self.prev.clone())
            } else {
                let stack_top = state.stack.last().cloned().unwrap_or(Rc::new(NullIota));
                new_acc.borrow_mut().push_back(stack_top.clone())
            }
        }

        if self.index >= self.collect.1 {
            //if frame is last in range, apply map
            state.stack = vector![];

            if self.maps.clone().is_empty() {
                state.stack = base_stack.clone();

                let result = Rc::new(self.acc.borrow().clone());
                if self.collect_single {
                    state.stack.push_back(result[0].clone());
                } else {
                    state.stack.push_back(result);
                }
            } else {
                state
                    .continuation
                    .push_back(ContinuationFrame::Map(FrameMap {
                        data: self.acc.borrow().clone(),
                        maps: self.maps.clone(),
                        base_stack: base_stack.clone(),
                        acc: Rc::new(RefCell::new(vector![])),
                        init: true,
                        current_map: vector![],
                        collect_single: self.collect_single,
                    }));
            }

            Ok(())
        } else {
            //else push next frames
            let result: Rc<dyn Iota> = if self.base_stack.is_none() {
                self.prev.clone()
            } else {
                state.stack.last().cloned().unwrap_or(Rc::new(NullIota))
            };

            state.stack = vector![];
            state.stack.push_back(result.clone());

            state
                .continuation
                .push_back(ContinuationFrame::Iterate(FrameIterate {
                    base_stack: Some(base_stack),
                    index: self.index + 1,
                    acc: new_acc.clone(),
                    prev: result,
                    ..self.clone()
                }));

            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: self.gen_next_code.clone(),
                }));

            Ok(())
        }
    }

    fn break_out(&self, state: &mut State) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct FrameMap {
    pub data: Vector<Rc<dyn Iota>>,
    pub maps: Vector<Vector<AstNode>>,
    pub current_map: Vector<AstNode>,
    pub base_stack: Vector<Rc<dyn Iota>>,
    pub acc: ThothAcc,
    pub init: bool,
    pub collect_single: bool,
}

impl ContinuationFrameTrait for FrameMap {
    fn evaluate(
        &self,
        state: &mut State,
        _: &PatternRegistry,
        macros: &Macros,
    ) -> Result<(), (Mishap, Location)> {
        if self.init {
            let mut new_maps = self.maps.clone();
            let current_map = new_maps.pop_front().unwrap();

            let mut new_data = self.data.clone();
            let element = new_data.pop_front().unwrap();
            state
                .continuation
                .push_back(ContinuationFrame::Map(FrameMap {
                    data: new_data,
                    maps: new_maps,
                    base_stack: self.base_stack.clone(),
                    acc: self.acc.clone(),
                    init: false,
                    current_map: current_map.clone(),
                    collect_single: self.collect_single,
                }));

            state.stack = vector![element];

            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: current_map.clone(),
                }));

            return Ok(());
        }

        //end of map
        if self.data.is_empty() {
            let new_acc: Rc<RefCell<Vector<Rc<dyn Iota>>>> = self.acc.clone();
            let stack_top = state.stack.last().cloned().unwrap_or(Rc::new(NullIota));
            new_acc.borrow_mut().push_back(stack_top);

            //if there are more maps
            if self.maps.is_empty().not() {
                state.stack = vector![];

                state
                    .continuation
                    .push_back(ContinuationFrame::Map(FrameMap {
                        data: self.acc.borrow().clone(),
                        maps: self.maps.clone(),
                        base_stack: self.base_stack.clone(),
                        acc: Rc::new(RefCell::new(vector![])),
                        init: true,
                        current_map: vector![],
                        collect_single: self.collect_single,
                    }));
            //end of all maps
            } else {
                state.stack = self.base_stack.clone();

                let result = Rc::new(self.acc.borrow().clone());

                if self.collect_single {
                    state.stack.push_back(result[0].clone());
                } else {
                    state.stack.push_back(result);
                }
            }
            Ok(())
        //iter next
        } else {
            let mut new_data = self.data.clone();
            let stack_top = state.stack.last().cloned().unwrap_or(Rc::new(NullIota));
            let element = new_data.pop_front().unwrap();
            let new_acc: Rc<RefCell<Vector<Rc<dyn Iota>>>> = self.acc.clone();

            new_acc.borrow_mut().push_back(stack_top);

            state
                .continuation
                .push_back(ContinuationFrame::Map(FrameMap {
                    data: new_data,
                    maps: self.maps.clone(),
                    base_stack: self.base_stack.clone(),
                    acc: new_acc,
                    init: false,
                    current_map: self.current_map.clone(),
                    collect_single: self.collect_single,
                }));

            state.stack = vector![element];

            state
                .continuation
                .push_back(ContinuationFrame::Evaluate(FrameEvaluate {
                    nodes_queue: self.current_map.clone(),
                }));

            Ok(())
        }
    }

    fn break_out(&self, state: &mut State) -> bool {
        true
    }
}

pub fn iota_list_to_ast_node_list(list: Rc<Vector<Rc<dyn Iota>>>) -> Vector<AstNode> {
    list.iter()
        .enumerate()
        .map(
            |(index, iota)| match iota.clone().downcast_rc::<PatternIota>() {
                Ok(pattern) => AstNode::Action {
                    location: pattern.location.clone(),
                    name: pattern.signature.as_str(),
                    value: *pattern.value.clone(),
                },
                Err(_) => AstNode::Op {
                    location: Location::List(index),
                    name: OpName::Embed,
                    arg: Some(OpValue::Iota(iota.clone())),
                },
            },
        )
        .collect()
}
