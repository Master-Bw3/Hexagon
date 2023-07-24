use std::{collections::HashMap, rc::Rc, vec};

use crate::{
    interpreter::{mishap::Mishap, ops::EmbedType},
    iota::{hex_casting::pattern::PatternIota, Iota},
    parser::{ActionValue, OpValue},
    pattern_registry::PatternRegistry,
};

pub fn compile_op_copy(
    heap: &mut HashMap<String, i32>,
    pattern_registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let mut compiled: Vec<Rc<dyn Iota>> = vec![Rc::new(
        PatternIota::from_name(pattern_registry, "duplicate", None, None).unwrap(),
    )];

    compiled.append(&mut compile_op_store(heap, pattern_registry, arg)?);

    Ok(compiled)
}

pub fn compile_op_store(
    heap: &mut HashMap<String, i32>,
    registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let (index, var) = {
        match value {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => (heap.get(var), var),
        }
    };

    let compiled: Vec<Rc<dyn Iota>> = match index {
        Some(index) => {
            vec![
                Rc::new(PatternIota::from_name(registry, "read/local", None, None).unwrap()),
                Rc::new(
                    PatternIota::from_name(
                        registry,
                        "number",
                        Some(ActionValue::Iota(Rc::new(*index as f64))),
                        None,
                    )
                    .unwrap(),
                ),
                Rc::new(PatternIota::from_name(registry, "rotate", None, None).unwrap()),
                Rc::new(PatternIota::from_name(registry, "modify_in_place", None, None).unwrap()),
                Rc::new(PatternIota::from_name(registry, "write/local", None, None).unwrap()),
            ]
        }
        None => {
            let new_index = heap.values().fold(0, |acc, val| i32::max(acc, *val));
            heap.insert(var.clone(), new_index);

            vec![
                Rc::new(PatternIota::from_name(registry, "read/local", None, None).unwrap()),
                Rc::new(PatternIota::from_name(registry, "swap", None, None).unwrap()),
                Rc::new(PatternIota::from_name(registry, "append", None, None).unwrap()),
                Rc::new(PatternIota::from_name(registry, "write/local", None, None).unwrap()),
            ]
        }
    };

    Ok(compiled)
}

pub fn compile_op_push(
    heap: &mut HashMap<String, i32>,
    registry: &PatternRegistry,
    arg: &Option<OpValue>,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let index = {
        match value {
            OpValue::Iota(iota) => Err(Mishap::OpExpectedVar(iota.clone()))?,
            OpValue::Var(var) => heap
                .get(var)
                .ok_or(Mishap::VariableNotAssigned(var.clone()))?,
        }
    };
    let compiled: Vec<Rc<dyn Iota>> = vec![
        Rc::new(PatternIota::from_name(registry, "read/local", None, None).unwrap()),
        Rc::new(
            PatternIota::from_name(
                registry,
                "number",
                Some(ActionValue::Iota(Rc::new(*index as f64))),
                None
            )
            .unwrap(),
        ),
        Rc::new(PatternIota::from_name(registry, "index", None, None).unwrap()),
    ];

    Ok(compiled)
}

pub fn compile_op_embed(
    registry: &PatternRegistry,
    depth: u32,
    arg: &Option<OpValue>,
    embed_type: EmbedType,
) -> Result<Vec<Rc<dyn Iota>>, Mishap> {
    let value = arg.as_ref().ok_or(Mishap::OpNotEnoughArgs(1))?;

    let iota = {
        if let OpValue::Iota(iota) = value {
            iota.clone()
        } else {
            Err(Mishap::OpExpectedIota)?
        }
    };

    //handle smart embed
    let embed_type = match embed_type {
        EmbedType::Smart => {
            if depth > 0 {
                EmbedType::IntroRetro
            } else {
                EmbedType::Consider
            }
        }
        _ => embed_type,
    };

    let compiled = match embed_type {
        EmbedType::Normal => vec![iota],
        EmbedType::Consider => {
            let consideration =
                Rc::new(PatternIota::from_name(registry, "escape", None, None).unwrap());

            let mut result: Vec<Rc<dyn Iota>> = vec![consideration; i32::pow(2, depth) as usize];
            result.push(iota);
            result
        }
        EmbedType::IntroRetro => vec![
            Rc::new(PatternIota::from_name(registry, "open_paren", None, None).unwrap()),
            iota,
            Rc::new(PatternIota::from_name(registry, "close_paren", None, None).unwrap()),
            Rc::new(PatternIota::from_name(registry, "splat", None, None).unwrap()),
        ],
        EmbedType::Smart => unreachable!(),
    };

    Ok(compiled)
}
