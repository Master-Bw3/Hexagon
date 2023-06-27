use crate::{
    interpreter::{mishap::Mishap, state::State},
    iota::Iota,
    parser::ActionValue,
};

#[derive(Clone)]

pub struct Pattern {
    pub display_name: String,
    pub internal_name: String,
    pub signature: String,
    pub action: fn(state: State, value: Option<Iota>) -> Result<State, Mishap>,
}

impl Pattern {
    pub fn new(
        display_name: &str,
        internal_name: &str,
        signature: &str,
        action: fn(state: State, value: Option<Iota>) -> Result<State, Mishap>,
    ) -> Pattern {
        Pattern {
            display_name: display_name.to_string(),
            internal_name: internal_name.to_string(),
            signature: signature.to_string(),
            action,
        }
    }

    pub fn operate(&self, state: State, value: Option<ActionValue>) -> Result<State, Mishap> {
        let value = match value {
            Some(val) => match val {
                ActionValue::Iota(iota) => Some(iota),
                ActionValue::Bookkeeper(_) => None,
            },
            None => None,
        };

        (self.action)(state, value)
    }
}
