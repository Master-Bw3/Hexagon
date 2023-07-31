use std::rc::Rc;

use im::Vector;

use crate::{interpreter::mishap::Mishap, iota::Iota};

pub type StringIota = String;

impl Iota for StringIota {
    fn display(&self) -> String {
        format!("{:?}", self)
    }

    fn display_type_name() -> String {
        "String".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<StringIota>() {
            Some(other) => self == other,
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        format!("{{\"hexcasting:type\": \"moreiotas:string\", \"hexcasting:data\": \"{self}\"}}")
    }
}

pub trait StringVecExt {
    fn is_string_vec(&self) -> bool;
}

impl StringVecExt for Vector<Rc<dyn Iota>> {
    fn is_string_vec(&self) -> bool {
        self.iter()
            .filter(|i| (**i).clone().downcast_rc::<StringIota>().is_ok())
            .count()
            == self.len()
    }
}
