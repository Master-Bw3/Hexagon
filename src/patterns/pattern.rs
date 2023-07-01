use crate::{
    interpreter::{mishap::Mishap, state::State},
    parser::ActionValue, pattern_registry::PatternRegistry,
};

pub struct Pattern {
    pub display_name: String,
    pub internal_name: String,
    pub signature: String,
    pub action: Box<dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>>,
}

impl Pattern {
    pub fn new(
        display_name: &str,
        internal_name: &str,
        signature: &str,
        action: &'static dyn for<'a> Fn(&'a mut State, &PatternRegistry) -> Result<&'a mut State, Mishap>,
    ) -> Pattern {
        Pattern {
            display_name: display_name.to_string(),
            internal_name: internal_name.to_string(),
            signature: signature.to_string(),
            action: Box::new(action),
        }
    }

    pub fn operate<'a>(&self, state: &'a mut State, pattern_registry: &PatternRegistry, _value: Option<ActionValue>) -> Result<&'a mut State, Mishap> {
        // let value = match value {
        //     Some(val) => match val {
        //         ActionValue::Iota(iota) => Some(iota),
        //         ActionValue::Bookkeeper(_) => None,
        //     },
        //     None => None,
        // };

        (self.action)(state, pattern_registry)
    }
}
