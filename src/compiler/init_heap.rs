use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::mishap::Mishap,
    iota::{hex_casting::pattern::PatternIota, Iota},
    pattern_registry::PatternRegistry, parser::ActionValue,
};

pub fn init_heap(
    heap: &HashMap<String, i32>,
    registry: &PatternRegistry,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let len = heap.len();
    let result: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "const/null", None, None).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(len as f64))),
                None,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "duplicate_n", None, None).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(len as f64))),
                None,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "last_n_list", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "write/local", None, None).unwrap()),
        ];

    Ok(result)
}
