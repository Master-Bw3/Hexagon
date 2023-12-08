use std::ops::Not;

use crate::{interpreter::continuation::Continuation, iota::Iota};

#[derive(Debug, Clone)]
pub struct ContinuationIota {
    pub value: Continuation,
}

impl Iota for ContinuationIota {
    fn display(&self) -> String {
        "Continuation".to_string()
    }

    fn display_type_name() -> String {
        "Continuation".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<ContinuationIota>() {
            Some(other) => {
                self.value.len() == other.value.len()
                    && self
                        .value.iter()
                        .zip(other.value.iter())
                        .map(|(_rhs, _lhs)| false) //TODO: fix this
                        .collect::<im::Vector<bool>>()
                        .contains(&false)
                        .not()
            }
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        todo!()
    }
}
