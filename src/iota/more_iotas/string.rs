use std::rc::Rc;

use im::Vector;

use crate::{interpreter::mishap::Mishap, iota::Iota};

pub type StringIota = String;

impl Iota for StringIota {
    fn display(&self) -> String {
        format!("{:?}", self)
    }
    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<StringIota>() {
            Some(other) => self == other,
            None => false,
        }
    }
}

pub trait StringVecExt {
    fn string_vec(&self, index: usize) -> Result<&Self, Mishap>;
}

impl StringVecExt for Vector<Rc<dyn Iota>> {
    fn string_vec(&self, index: usize) -> Result<&Self, Mishap> {
        if self
            .iter()
            .filter(|i| (**i).clone().downcast_rc::<StringIota>().is_ok())
            .count()
            == self.len()
        {
            Ok(self)
        } else {
            Err(Mishap::IncorrectIota(
                index,
                "List of Strings".to_string(),
                Rc::new(self.clone()),
            ))
        }
    }
}
