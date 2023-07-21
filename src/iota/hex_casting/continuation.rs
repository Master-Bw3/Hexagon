use std::ops::Not;

use crate::{interpreter::continuation::Continuation, iota::Iota};

#[derive(Debug)]
pub struct ContinuationIota {
    pub value: Continuation,
}

impl Iota for ContinuationIota {
    fn display(&self) -> String {
        "Continuation".to_string()
    }
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<ContinuationIota>() {
            Some(other) => {
                self.value.len() == other.value.len()
                    && self
                        .value.iter()
                        .zip(other.value.iter())
                        .map(|(rhs, lhs)| false) //todo: fix this
                        .collect::<im::Vector<bool>>()
                        .contains(&false)
                        .not()
            }
            None => false,
        }
    }
}
