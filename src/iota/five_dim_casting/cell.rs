use std::{cell::RefCell, rc::Rc};

use crate::iota::{self, Iota};

pub type CellIota = RefCell<Rc<dyn Iota>>;

impl Iota for CellIota {
    fn display(&self) -> String {
        format!("Cell({})", self.borrow().display())
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        match other.downcast_ref::<CellIota>() {
            Some(other) => Rc::ptr_eq(&self.clone().into_inner(), &other.clone().into_inner()),
            None => false,
        }
    }

    fn serialize_to_nbt(&self) -> String {
        "{\"hexcasting:type\": \"hexcasting:null\", \"hexcasting:data\": {}}".to_string()
    }

    fn display_type_name() -> String {
        "Cell".to_string()
    }
}
