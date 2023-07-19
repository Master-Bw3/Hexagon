use std::ops::Not;

use crate::{interpreter::continuation::Continuation, iota::Iota};

pub type ContinuationIota = Continuation;

impl Iota for ContinuationIota {
    fn display(&self) -> String {
        "Continuation".to_string()
    }
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<Continuation>() {
            Some(other) => {
                self.len() == other.len()
                    && self
                        .iter()
                        .zip(other.iter())
                        .map(|(rhs, lhs)| false) //todo: fix this
                        .collect::<im::Vector<bool>>()
                        .contains(&false)
                        .not()
            }
            None => false,
        }
    }
}
