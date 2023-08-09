use std::rc::Rc;

use crate::{
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{ActionValue, Location},
    pattern_registry::PatternRegistry,
};

pub fn compile_external(
    hex: &mut Vec<Rc<dyn Iota>>,
    registry: &PatternRegistry,
) -> Vec<Rc<dyn Iota>> {
    let mut result: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "open_paren", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "read/local", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "const/null", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "equals", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "open_paren", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "open_paren", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "mask",
                Some(ActionValue::Bookkeeper("-".to_string())),
                Location::Unknown,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "close_paren", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "splat", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "write/local", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "close_paren", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "empty_list", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "if", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "eval", None, Location::Unknown).unwrap()),
        Rc::new(PatternIota::from_name(registry, "close_paren", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(5.0))),
                Location::Unknown,
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "read/local", None, Location::Unknown).unwrap()),
        Rc::new(
            PatternIota::from_name(registry, "modify_in_place", None, Location::Unknown).unwrap(),
        ),
    ];

    result.append(hex);
    result.push(Rc::new(
        PatternIota::from_name(registry, "concat", None, Location::Unknown).unwrap(),
    ));

    result
}
