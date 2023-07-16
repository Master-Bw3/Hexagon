pub mod hex_casting;
pub mod more_iotas;
pub mod constructors;

use std::rc::Rc;

use crate::{
    interpreter::{mishap::Mishap, state::State},
    parser::ActionValue,
    pattern_registry::PatternRegistry,
};

pub type ActionNoValueType =
    dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>;

pub type ActionWithValueType = dyn for<'a> Fn(
    &'a mut State,
    &PatternRegistry,
    Option<&ActionValue>,
) -> Result<&'a mut State, Mishap>;

#[derive(Clone)]
pub enum ActionFunction {
    ActionNoValue(Rc<ActionNoValueType>),
    ActionWithValue(Rc<ActionWithValueType>),
}

#[derive(Clone)]
pub struct Pattern {
    pub display_name: String,
    pub internal_name: String,
    pub signature: String,
    pub action: ActionFunction,
}

impl Pattern {
    pub fn new(
        display_name: &str,
        internal_name: &str,
        signature: &str,
        action: Box<ActionNoValueType>,
    ) -> Pattern {
        Pattern {
            display_name: display_name.to_string(),
            internal_name: internal_name.to_string(),
            signature: signature.to_string(),
            action: ActionFunction::ActionNoValue(Rc::new(action)),
        }
    }

    pub fn new_with_val(
        display_name: &str,
        internal_name: &str,
        signature: &str,
        action: Box<ActionWithValueType>,
    ) -> Pattern {
        Pattern {
            display_name: display_name.to_string(),
            internal_name: internal_name.to_string(),
            signature: signature.to_string(),
            action: ActionFunction::ActionWithValue(Rc::new(action)),
        }
    }

    pub fn operate<'a>(
        &self,
        state: &'a mut State,
        pattern_registry: &PatternRegistry,
        value: &Option<ActionValue>,
    ) -> Result<&'a mut State, Mishap> {
        match &self.action {
            ActionFunction::ActionNoValue(action) => action(state, pattern_registry),
            ActionFunction::ActionWithValue(action) => {
                action(state, pattern_registry, value.as_ref())
            }
        }
    }
}
