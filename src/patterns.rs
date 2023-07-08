pub mod misc;
pub mod selectors;
pub mod math;
pub mod special;
pub mod eval;
pub mod constructors;
pub mod lists;
pub mod stack;
pub mod read_write;
pub mod swizzle;

use crate::{
    interpreter::{mishap::Mishap, state::State},
    parser::ActionValue,
    pattern_registry::PatternRegistry,
};

pub type ActionNoValueType =
    dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>;

pub type ActionWithValueType =
    dyn for<'a> Fn(&'a mut State, &PatternRegistry, &ActionValue) -> Result<&'a mut State, Mishap>;

pub enum ActionFunction {
    ActionNoValue(Box<ActionNoValueType>),
    ActionWithValue(Box<ActionWithValueType>),
}

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
            action: ActionFunction::ActionNoValue(Box::new(action)),
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
            action: ActionFunction::ActionWithValue(Box::new(action)),
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
            ActionFunction::ActionWithValue(action) => action(
                state,
                pattern_registry,
                value.as_ref().ok_or(Mishap::ExpectedValue)?,
            ),
        }
    }
}
