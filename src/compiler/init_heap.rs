use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::mishap::Mishap,
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{ActionValue, Location},
    pattern_registry::PatternRegistry,
};

pub fn init_heap(
    heap: &HashMap<String, i32>,
    registry: &PatternRegistry,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let len = heap.len();
    let result: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "empty_list", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "write/local", None, Location::Unknown).unwrap()),
    ];

    Ok(result)
}
