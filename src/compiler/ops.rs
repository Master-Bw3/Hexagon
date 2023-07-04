use std::{collections::HashMap, vec};

use crate::{
    iota::{Iota, PatternIota},
    parser::OpValue,
    pattern_registry::PatternRegistry,
};

pub fn compile_op_copy(
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Iota>, String> {
    let mut compiled = vec![Iota::Pattern(PatternIota::from_name(
        pattern_registry,
        "duplicate",
        None,
    ))];

    compiled.append(&mut compile_op_store(heap, pattern_registry, arg)?);

    Ok(compiled)
}

pub fn compile_op_store(
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Iota>, String> {
    let value = arg
        .as_ref()
        .ok_or("Expected 1 input, but recieved 0 inputs")?;

    let (index, var) = {
        match value {
            OpValue::Iota(iota) => Err(format!("Expected Var, recieved {:?}", iota))?,
            OpValue::Var(var) => (heap.get(var), var),
        }
    };

    let compiled = match index {
        Some(index) => {
            vec![
                Iota::Pattern(PatternIota::from_name(pattern_registry, "read/local", None)),
                Iota::Pattern(PatternIota::from_name(
                    pattern_registry,
                    "number",
                    Some(Iota::Number(*index as f32)),
                )),
                Iota::Pattern(PatternIota::from_name(pattern_registry, "rotate", None)),
                Iota::Pattern(PatternIota::from_name(
                    pattern_registry,
                    "modify_in_place",
                    None,
                )),
                Iota::Pattern(PatternIota::from_name(
                    pattern_registry,
                    "write/local",
                    None,
                )),
            ]
        }
        None => {
            let new_index = heap.values().fold(0, |acc, val| i32::max(acc, *val));
            heap.insert(var.clone(), new_index);

            vec![
                Iota::Pattern(PatternIota::from_name(pattern_registry, "read/local", None)),
                Iota::Pattern(PatternIota::from_name(pattern_registry, "swap", None)),
                Iota::Pattern(PatternIota::from_name(pattern_registry, "append", None)),
                Iota::Pattern(PatternIota::from_name(
                    pattern_registry,
                    "write/local",
                    None,
                )),
            ]
        }
    };

    Ok(compiled)
}

pub fn compile_op_push(
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Iota>, String> {
    let value = arg
        .as_ref()
        .ok_or("Expected 1 input, but recieved 0 inputs")?;

    let index = {
        match value {
            OpValue::Iota(iota) => Err(format!("Expected Var, recieved {:?}", iota))?,
            OpValue::Var(var) => heap.get(var).ok_or("variable not assigned".to_string())?,
        }
    };
    let compiled = vec![
        Iota::Pattern(PatternIota::from_name(pattern_registry, "read/local", None)),
        Iota::Pattern(PatternIota::from_name(
            pattern_registry,
            "number",
            Some(Iota::Number(*index as f32)),
        )),
        Iota::Pattern(PatternIota::from_name(pattern_registry, "index", None)),
    ];

    Ok(compiled)
}
