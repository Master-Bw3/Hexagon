use std::rc::Rc;

use crate::{iota::{Iota, hex_casting::pattern::PatternIota}, pattern_registry::PatternRegistry, parser::ActionValue};

pub fn compile_external(hex: &mut Vec<Rc<dyn Iota>>, registry: &PatternRegistry) -> Vec<Rc<dyn Iota>> {
    let mut result: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "open_paren", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "read/local", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "const/null", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "equals", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "open_paren", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "open_paren", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "mask", Some(ActionValue::Bookkeeper("-".to_string())), None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "close_paren", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "splat", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "write/local", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "close_paren", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "empty_list", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "if", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "eval", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "close_paren", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "number", Some(ActionValue::Iota(Rc::new(6.0))), None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "read/local", None, None).unwrap()),
        Rc::new(PatternIota::from_name(registry, "modify_in_place", None, None).unwrap()),

    ];        

    result.append(hex);
    result.push(Rc::new(PatternIota::from_name(registry, "concat", None, None).unwrap()));
    
    result
}