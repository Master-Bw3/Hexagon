use std::{collections::HashMap, ops::Not, rc::Rc};

use im::Vector;

use crate::{
    interpreter::{state::{Entity, EntityType, Stack}, continuation::Continuation},
    iota::{Iota, hex_casting::list::ListIota}, parser::AstNode,
};

#[derive(Debug, Clone)]
pub struct ContinuumIota {
    pub front_val: Rc<dyn Iota>,
    pub gen_next_func: Vector<AstNode>,
    pub maps: Vector<Vector<AstNode>>
}


impl Iota for ContinuumIota {
    fn display(&self) -> String {
        format!("[{}, ...]", self.front_val.display())
    }

    fn display_type_name() -> String {
        "Continuum".to_string()
    }

    fn tolerates_other(&self, other: &dyn Iota) -> bool {
        false
    }

    fn serialize_to_nbt(&self) -> String {
        "".to_string()
    }
}